use axum::{
	Router,
	body::Body,
	http::{HeaderMap, HeaderValue, Method, Request, StatusCode, header},
	response::{IntoResponse, Response},
};
use chrono::TimeDelta;
use std::{
	collections::{BTreeMap, HashMap},
	convert::Infallible,
	net::SocketAddr,
	pin::Pin,
	sync::Arc,
	task::{Context, Poll},
	time::Instant,
};
use tower::Service;
use tracing::trace;

use crate::{
	ClientInfo, RenderContext, Rendered, RenderedBody,
	mime::MimeType,
	servable::{Servable, ServableWithRoute},
};

struct Default404 {}

impl Servable for Default404 {
	fn head<'a>(
		&'a self,
		_ctx: &'a RenderContext,
	) -> Pin<Box<dyn Future<Output = Rendered<()>> + 'a + Send + Sync>> {
		Box::pin(async {
			return Rendered {
				code: StatusCode::NOT_FOUND,
				body: (),
				ttl: Some(TimeDelta::days(1)),
				immutable: true,
				headers: HeaderMap::new(),
				mime: Some(MimeType::Html),
			};
		})
	}

	fn render<'a>(
		&'a self,
		ctx: &'a RenderContext,
	) -> Pin<Box<dyn Future<Output = Rendered<RenderedBody>> + 'a + Send + Sync>> {
		Box::pin(async { self.head(ctx).await.with_body(RenderedBody::Empty) })
	}
}

/// A set of related [Servable]s under one route.
///
/// Use as follows:
/// ```rust
/// use servable::{ServableRouter, StaticAsset, mime::MimeType};
/// use axum::Router;
/// use tower_http::compression::{CompressionLayer, predicate::DefaultPredicate};
///
/// // Add compression, for example.
/// // Also consider CORS and timeout.
/// let compression: CompressionLayer = CompressionLayer::new()
/// 	.br(true)
/// 	.deflate(true)
/// 	.gzip(true)
/// 	.zstd(true)
/// 	.compress_when(DefaultPredicate::new());
///
/// let route = ServableRouter::new()
/// 	.add_page(
/// 		"/page",
/// 		StaticAsset {
/// 			bytes: "I am a page".as_bytes(),
/// 			mime: MimeType::Text,
/// 		},
/// 	);
///
/// let router: Router<()> = route
/// 	.into_router()
/// 	.layer(compression.clone());
/// ```
#[derive(Clone)]
pub struct ServableRouter {
	pages: Arc<HashMap<String, Arc<dyn Servable>>>,
	notfound: Arc<dyn Servable>,
}

impl ServableRouter {
	/// Create a new, empty [ServableRouter]
	#[inline(always)]
	pub fn new() -> Self {
		Self {
			pages: Arc::new(HashMap::new()),
			notfound: Arc::new(Default404 {}),
		}
	}

	/// Set this server's "not found" page
	#[inline(always)]
	pub fn with_404<S: Servable + 'static>(mut self, page: S) -> Self {
		self.notfound = Arc::new(page);
		self
	}

	/// Add a [Servable] to this server at the given route.
	/// - panics if route does not start with a `/`, ends with a `/`, or contains `//`.
	///   - urls are normalized, routes that violate this condition will never be served.
	///   - `/` is an exception, it is valid.
	/// - panics if called after this service is started
	/// - overwrites existing pages
	#[inline(always)]
	pub fn add_page<S: Servable + 'static>(mut self, route: impl Into<String>, page: S) -> Self {
		let route = route.into();

		if !route.starts_with("/") {
			panic!("route must start with /")
		};

		if route.ends_with("/") && route != "/" {
			panic!("route must not end with /")
		};

		if route.contains("//") {
			panic!("route must not contain //")
		};

		#[expect(clippy::expect_used)]
		Arc::get_mut(&mut self.pages)
			.expect("add_pages called after service was started")
			.insert(route, Arc::new(page));

		self
	}

	/// Add a [ServableWithRoute] to this server.
	/// Behaves exactly like [Self::add_page].
	#[inline(always)]
	pub fn add_page_with_route<S: Servable + 'static>(
		self,
		servable_with_route: &'static ServableWithRoute<S>,
	) -> Self {
		self.add_page(servable_with_route.route(), servable_with_route)
	}

	/// Convenience method.
	/// Turns this service into a router.
	///
	/// Equivalent to:
	/// ```ignore
	/// Router::new().fallback_service(self)
	/// ```
	#[inline(always)]
	pub fn into_router<T: Clone + Send + Sync + 'static>(self) -> Router<T> {
		Router::new().fallback_service(self)
	}
}

//
// MARK: impl Service
//

impl Service<Request<Body>> for ServableRouter {
	type Response = Response;
	type Error = Infallible;
	type Future =
		Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

	fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		Poll::Ready(Ok(()))
	}

	fn call(&mut self, req: Request<Body>) -> Self::Future {
		if req.method() != Method::GET && req.method() != Method::HEAD {
			let mut headers = HeaderMap::with_capacity(1);
			headers.insert(header::ACCEPT, HeaderValue::from_static("GET,HEAD"));
			return Box::pin(async {
				Ok((StatusCode::METHOD_NOT_ALLOWED, headers).into_response())
			});
		}

		let pages = self.pages.clone();
		let notfound = self.notfound.clone();
		Box::pin(async move {
			let addr = req.extensions().get::<SocketAddr>().copied();
			let route = req.uri().path().to_owned();
			let headers = req.headers().clone();
			let query: BTreeMap<String, String> =
				serde_urlencoded::from_str(req.uri().query().unwrap_or("")).unwrap_or_default();

			let start = Instant::now();
			let client_info = ClientInfo::from_headers(&headers);
			let ua = headers
				.get("user-agent")
				.and_then(|x| x.to_str().ok())
				.unwrap_or("");

			trace!(
				message = "Serving route",
				route,
				addr = ?addr,
				user_agent = ua,
				device_type = ?client_info.device_type
			);

			// Normalize url with redirect
			if (route.ends_with('/') && route != "/") || route.contains("//") {
				let mut new_route = route.clone();
				while new_route.contains("//") {
					new_route = new_route.replace("//", "/");
				}
				let new_route = new_route.trim_matches('/');

				trace!(
					message = "Redirecting",
					route,
					new_route,
					addr = ?addr,
					user_agent = ua,
					device_type = ?client_info.device_type
				);

				let mut headers = HeaderMap::with_capacity(1);
				match HeaderValue::from_str(&format!("/{new_route}")) {
					Ok(x) => headers.append(header::LOCATION, x),
					Err(_) => return Ok(StatusCode::BAD_REQUEST.into_response()),
				};
				return Ok((StatusCode::PERMANENT_REDIRECT, headers).into_response());
			}

			let ctx = RenderContext {
				client_info,
				route,
				query,
			};

			let page = pages.get(&ctx.route).unwrap_or(&notfound);
			let mut rend = match req.method() == Method::HEAD {
				true => page.head(&ctx).await.with_body(RenderedBody::Empty),
				false => page.render(&ctx).await,
			};

			// Tweak headers
			{
				if !rend.headers.contains_key(header::CACHE_CONTROL) {
					let max_age = rend.ttl.map(|x| x.num_seconds()).unwrap_or(1).max(1);

					let mut value = String::new();
					if rend.immutable {
						value.push_str("immutable, ");
					}

					value.push_str("public, ");
					value.push_str(&format!("max-age={}, ", max_age));

					#[expect(clippy::unwrap_used)]
					rend.headers.insert(
						header::CACHE_CONTROL,
						HeaderValue::from_str(value.trim().trim_end_matches(',')).unwrap(),
					);
				}

				if !rend.headers.contains_key("Accept-CH") {
					rend.headers
						.insert("Accept-CH", HeaderValue::from_static("Sec-CH-UA-Mobile"));
				}

				if !rend.headers.contains_key(header::CONTENT_TYPE)
					&& let Some(mime) = &rend.mime
				{
					#[expect(clippy::unwrap_used)]
					rend.headers.insert(
						header::CONTENT_TYPE,
						HeaderValue::from_str(&mime.to_string()).unwrap(),
					);
				}
			}

			trace!(
				message = "Served route",
				route = ctx.route,
				addr = ?addr,
				user_agent = ua,
				device_type = ?client_info.device_type,
				time_ns = start.elapsed().as_nanos()
			);

			Ok(match rend.body {
				RenderedBody::Static(d) => (rend.code, rend.headers, d).into_response(),
				RenderedBody::Bytes(d) => (rend.code, rend.headers, d).into_response(),
				RenderedBody::String(s) => (rend.code, rend.headers, s).into_response(),
				RenderedBody::Empty => (rend.code, rend.headers).into_response(),
			})
		})
	}
}
