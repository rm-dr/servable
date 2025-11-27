use std::pin::Pin;

use axum::http::{
	HeaderMap, HeaderValue, StatusCode,
	header::{self, InvalidHeaderValue},
};

use crate::{RenderContext, Rendered, RenderedBody, servable::Servable};

#[expect(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectCode {
	/// Reply with an http 307 (temporary redirect)
	Http307,

	/// Reply with an http 308 (permanent redirect)
	Http308,
}

/// A simple http edirect
pub struct Redirect {
	to: HeaderValue,
	code: RedirectCode,
}

impl Redirect {
	/// Create a new [Redirect] to the given route.
	/// Returns an http 308 (permanent redirect)
	pub fn new(to: impl Into<String>) -> Result<Self, InvalidHeaderValue> {
		Ok(Self {
			to: HeaderValue::from_str(&to.into())?,
			code: RedirectCode::Http308,
		})
	}

	/// Create a new [Redirect] to the given route.
	/// Returns an http 307 (temporary redirect)
	pub fn new_307(to: impl Into<String>) -> Result<Self, InvalidHeaderValue> {
		Ok(Self {
			to: HeaderValue::from_str(&to.into())?,
			code: RedirectCode::Http307,
		})
	}
}

impl Servable for Redirect {
	fn head<'a>(
		&'a self,
		_ctx: &'a RenderContext,
	) -> Pin<Box<dyn Future<Output = Rendered<()>> + 'a + Send + Sync>> {
		Box::pin(async {
			let mut headers = HeaderMap::with_capacity(1);
			headers.append(header::LOCATION, self.to.clone());

			return Rendered {
				code: match self.code {
					RedirectCode::Http307 => StatusCode::TEMPORARY_REDIRECT,
					RedirectCode::Http308 => StatusCode::PERMANENT_REDIRECT,
				},
				headers,
				body: (),
				ttl: None,
				immutable: true,
				mime: None,
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
