use axum::http::{HeaderMap, StatusCode};
use chrono::TimeDelta;
use std::pin::Pin;

use crate::{RenderContext, Rendered, RenderedBody, mime::MimeType, servable::Servable};

/// A static blob of bytes
pub struct StaticAsset {
	/// The data to return
	pub bytes: &'static [u8],

	/// The type of `bytes`
	pub mime: MimeType,

	/// How long to cache this response.
	/// If None, never cache
	pub ttl: Option<TimeDelta>,
}

impl StaticAsset {
	/// Default ttl of a [StaticAsset]
	pub const DEFAULT_TTL: Option<TimeDelta> = Some(TimeDelta::days(14));

	/// Set `self.ttl`
	pub const fn with_ttl(mut self, ttl: Option<TimeDelta>) -> Self {
		self.ttl = ttl;
		self
	}
}

#[cfg(feature = "image")]
impl Servable for StaticAsset {
	fn head<'a>(
		&'a self,
		ctx: &'a RenderContext,
	) -> Pin<Box<dyn Future<Output = Rendered<()>> + 'a + Send + Sync>> {
		Box::pin(async {
			use crate::transform::TransformerChain;
			use std::str::FromStr;

			let is_image = TransformerChain::mime_is_image(&self.mime);

			let transform = match (is_image, ctx.query.get("t")) {
				(false, _) | (_, None) => None,

				(true, Some(x)) => match TransformerChain::from_str(x) {
					Ok(x) => Some(x),
					Err(_err) => {
						return Rendered {
							code: StatusCode::BAD_REQUEST,
							body: (),
							ttl: self.ttl,
							private: false,

							headers: HeaderMap::new(),
							mime: None,
						};
					}
				},
			};

			match transform {
				Some(transform) => {
					return Rendered {
						code: StatusCode::OK,
						body: (),
						ttl: self.ttl,
						private: false,

						headers: HeaderMap::new(),
						mime: Some(
							transform
								.output_mime(&self.mime)
								.unwrap_or(self.mime.clone()),
						),
					};
				}

				None => {
					return Rendered {
						code: StatusCode::OK,
						body: (),
						ttl: self.ttl,
						private: false,

						headers: HeaderMap::new(),
						mime: Some(self.mime.clone()),
					};
				}
			}
		})
	}

	fn render<'a>(
		&'a self,
		ctx: &'a RenderContext,
	) -> Pin<Box<dyn Future<Output = Rendered<RenderedBody>> + 'a + Send + Sync>> {
		Box::pin(async {
			use crate::transform::TransformerChain;
			use std::str::FromStr;
			use tracing::{error, trace};

			// Automatically provide transformation if this is an image
			let is_image = TransformerChain::mime_is_image(&self.mime);

			let transform = match (is_image, ctx.query.get("t")) {
				(false, _) | (_, None) => None,

				(true, Some(x)) => match TransformerChain::from_str(x) {
					Ok(x) => Some(x),
					Err(err) => {
						return Rendered {
							code: StatusCode::BAD_REQUEST,
							body: RenderedBody::String(err),
							ttl: self.ttl,
							private: false,

							headers: HeaderMap::new(),
							mime: None,
						};
					}
				},
			};

			match transform {
				Some(transform) => {
					trace!(message = "Transforming image", ?transform);

					let task = {
						let mime = Some(self.mime.clone());
						let bytes = self.bytes;
						tokio::task::spawn_blocking(move || {
							transform.transform_bytes(bytes, mime.as_ref())
						})
					};

					let res = match task.await {
						Ok(x) => x,
						Err(error) => {
							error!(message = "Error while transforming image", ?error);
							return Rendered {
								code: StatusCode::INTERNAL_SERVER_ERROR,
								body: RenderedBody::String(format!(
									"Error while transforming image: {error:?}"
								)),
								ttl: None,
								private: false,

								headers: HeaderMap::new(),
								mime: None,
							};
						}
					};

					match res {
						Ok((mime, bytes)) => {
							return Rendered {
								code: StatusCode::OK,
								body: RenderedBody::Bytes(bytes),
								ttl: self.ttl,
								private: false,

								headers: HeaderMap::new(),
								mime: Some(mime),
							};
						}

						Err(err) => {
							return Rendered {
								code: StatusCode::INTERNAL_SERVER_ERROR,
								body: RenderedBody::String(format!("{err}")),
								ttl: self.ttl,
								private: false,

								headers: HeaderMap::new(),
								mime: None,
							};
						}
					}
				}

				None => {
					return Rendered {
						code: StatusCode::OK,
						body: RenderedBody::Static(self.bytes),
						ttl: self.ttl,
						private: false,

						headers: HeaderMap::new(),
						mime: Some(self.mime.clone()),
					};
				}
			}
		})
	}
}

#[cfg(not(feature = "image"))]
impl Servable for StaticAsset {
	fn head<'a>(
		&'a self,
		_ctx: &'a RenderContext,
	) -> Pin<Box<dyn Future<Output = Rendered<()>> + 'a + Send + Sync>> {
		Box::pin(async {
			return Rendered {
				code: StatusCode::OK,
				body: (),
				ttl: self.ttl,
				private: false,

				headers: HeaderMap::new(),
				mime: Some(self.mime.clone()),
			};
		})
	}

	fn render<'a>(
		&'a self,
		ctx: &'a RenderContext,
	) -> Pin<Box<dyn Future<Output = Rendered<RenderedBody>> + 'a + Send + Sync>> {
		Box::pin(async {
			self.head(ctx)
				.await
				.with_body(RenderedBody::Static(self.bytes))
		})
	}
}
