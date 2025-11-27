//! This module provides the [Servable] trait,
//! as well as a few helper structs that implement it.

mod asset;

pub use asset::*;

mod html;
pub use html::*;

mod redirect;
pub use redirect::*;

/// Something that may be served over http. If implementing this trait,
/// refer to sample implementations in [redirect::Redirect], [asset::StaticAsset] and [html::HtmlPage].
pub trait Servable: Send + Sync {
	/// Return the same response as [Servable::render], but with an empty body.
	///
	/// This method is used to respond to `HEAD` requests.
	fn head<'a>(
		&'a self,
		ctx: &'a crate::RenderContext,
	) -> std::pin::Pin<Box<dyn Future<Output = crate::Rendered<()>> + 'a + Send + Sync>>;

	/// Render this page. Must return the same metadata as [Servable::head].
	/// Consider using [crate::Rendered::with_body] and [Servable::head] to implement this fn.
	///
	/// This method is used to respond to `GET` requests.
	fn render<'a>(
		&'a self,
		ctx: &'a crate::RenderContext,
	) -> std::pin::Pin<
		Box<dyn Future<Output = crate::Rendered<crate::RenderedBody>> + 'a + Send + Sync>,
	>;
}

//
// MARK: ServableWithRoute
//

/// A [Servable] and the route it is available at
pub struct ServableWithRoute<S: Servable> {
	/// The resource
	servable: S,

	/// The route this resource is available at
	route: std::sync::LazyLock<String>,
}

impl<S: Servable> ServableWithRoute<S> {
	/// Create a new [ServableWithRoute]
	pub const fn new(route_init: fn() -> std::string::String, servable: S) -> Self {
		Self {
			servable,
			route: std::sync::LazyLock::new(route_init),
		}
	}

	/// Get the route associated with this resource
	pub fn route(&self) -> &str {
		&self.route
	}
}

impl<S: Servable> Servable for ServableWithRoute<S> {
	#[inline(always)]
	fn head<'a>(
		&'a self,
		ctx: &'a crate::RenderContext,
	) -> std::pin::Pin<Box<dyn Future<Output = crate::Rendered<()>> + 'a + Send + Sync>> {
		self.servable.head(ctx)
	}

	#[inline(always)]
	fn render<'a>(
		&'a self,
		ctx: &'a crate::RenderContext,
	) -> std::pin::Pin<
		Box<dyn Future<Output = crate::Rendered<crate::RenderedBody>> + 'a + Send + Sync>,
	> {
		self.servable.render(ctx)
	}
}

impl<S: Servable> Servable for &'static S {
	#[inline(always)]
	fn head<'a>(
		&'a self,
		ctx: &'a crate::RenderContext,
	) -> std::pin::Pin<Box<dyn Future<Output = crate::Rendered<()>> + 'a + Send + Sync>> {
		(*self).head(ctx)
	}

	#[inline(always)]
	fn render<'a>(
		&'a self,
		ctx: &'a crate::RenderContext,
	) -> std::pin::Pin<
		Box<dyn Future<Output = crate::Rendered<crate::RenderedBody>> + 'a + Send + Sync>,
	> {
		(*self).render(ctx)
	}
}

impl<S: Servable> Servable for std::sync::LazyLock<S> {
	#[inline(always)]
	fn head<'a>(
		&'a self,
		ctx: &'a crate::RenderContext,
	) -> std::pin::Pin<Box<dyn Future<Output = crate::Rendered<()>> + 'a + Send + Sync>> {
		(**self).head(ctx)
	}

	#[inline(always)]
	fn render<'a>(
		&'a self,
		ctx: &'a crate::RenderContext,
	) -> std::pin::Pin<
		Box<dyn Future<Output = crate::Rendered<crate::RenderedBody>> + 'a + Send + Sync>,
	> {
		(**self).render(ctx)
	}
}
