use axum::http::{HeaderMap, StatusCode};
use chrono::TimeDelta;
use maud::{DOCTYPE, Markup, PreEscaped, html};
use serde::Deserialize;
use std::{hash::Hash, pin::Pin, sync::Arc};

use crate::{RenderContext, Rendered, RenderedBody, mime::MimeType, servable::Servable};

#[expect(missing_docs)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct PageMetadata {
	/// The title of this page.
	/// Browsers display this on the page's tab.
	pub title: String,

	/// The page author (metadata only)
	pub author: Option<String>,

	/// The page description (metadata only)
	pub description: Option<String>,

	/// The page image.
	/// Browsers display this on the page's tab.
	pub image: Option<String>,
}

impl Default for PageMetadata {
	fn default() -> Self {
		Self {
			title: "Untitled page".into(),
			author: None,
			description: None,
			image: None,
		}
	}
}

#[expect(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScriptSource<S> {
	/// Raw script data
	Inline(S),

	/// Load script from a url
	Linked(S),
}

/// A complete, dynamically-rendered blob of HTML.
#[derive(Clone)]
pub struct HtmlPage {
	/// This page's metadata
	pub meta: PageMetadata,

	/// If true, the contents of this page never change
	pub private: bool,

	/// How long this page's html may be cached.
	/// This controls the maximum age of a page shown to the user.
	///
	/// If `None`, this page is never cached.
	pub ttl: Option<TimeDelta>,

	/// A function that generates this page's html.
	///
	/// This should return the contents of this page's <body> tag,
	/// or the contents of a wrapper element (defined in the page server struct).
	///
	/// This closure must never return `<html>` or `<head>`.
	pub render: Arc<
		dyn Send
			+ Sync
			+ 'static
			+ for<'a> Fn(
				&'a HtmlPage,
				&'a RenderContext,
			) -> Pin<Box<dyn Future<Output = Markup> + Send + Sync + 'a>>,
	>,

	/// The response code that should accompany this html
	pub response_code: StatusCode,

	/// Scripts to include in this page. Order is preserved.
	pub scripts: Vec<ScriptSource<String>>,

	/// Styles to include in this page. Order is preserved.
	pub styles: Vec<ScriptSource<String>>,

	/// `name`, `content` for extra `<meta>` tags
	pub extra_meta: Vec<(String, String)>,
}

impl Default for HtmlPage {
	fn default() -> Self {
		HtmlPage {
			// No cache by default
			ttl: None,
			private: false,

			meta: Default::default(),
			render: Arc::new(|_, _| Box::pin(async { html!() })),
			response_code: StatusCode::OK,
			scripts: Vec::new(),
			styles: Vec::new(),
			extra_meta: Vec::new(),
		}
	}
}

impl HtmlPage {
	/// Set `self.meta`
	#[inline(always)]
	pub fn with_meta(mut self, meta: PageMetadata) -> Self {
		self.meta = meta;
		self
	}

	/// Set `self.generate`
	#[inline(always)]
	pub fn with_render<
		R: Send
			+ Sync
			+ 'static
			+ for<'a> Fn(
				&'a HtmlPage,
				&'a RenderContext,
			) -> Pin<Box<dyn Future<Output = Markup> + Send + Sync + 'a>>,
	>(
		mut self,
		render: R,
	) -> Self {
		self.render = Arc::new(render);
		self
	}

	/// Set `self.private`
	#[inline(always)]
	pub fn with_private(mut self, private: bool) -> Self {
		self.private = private;
		self
	}

	/// Set `self.html_ttl`
	#[inline(always)]
	pub fn with_ttl(mut self, ttl: Option<TimeDelta>) -> Self {
		self.ttl = ttl;
		self
	}

	/// Set `self.response_code`
	#[inline(always)]
	pub fn with_code(mut self, response_code: StatusCode) -> Self {
		self.response_code = response_code;
		self
	}

	/// Add an inline script to this page (after existing scripts)
	#[inline(always)]
	pub fn with_script_inline(mut self, script: impl Into<String>) -> Self {
		self.scripts.push(ScriptSource::Inline(script.into()));
		self
	}

	/// Add a linked script to this page (after existing scripts)
	#[inline(always)]
	pub fn with_script_linked(mut self, url: impl Into<String>) -> Self {
		self.scripts.push(ScriptSource::Linked(url.into()));
		self
	}

	/// Add a script to this page (after existing scripts)
	#[inline(always)]
	pub fn with_script(mut self, script: ScriptSource<impl Into<String>>) -> Self {
		let script = match script {
			ScriptSource::Inline(x) => ScriptSource::Inline(x.into()),
			ScriptSource::Linked(x) => ScriptSource::Linked(x.into()),
		};

		self.scripts.push(script);
		self
	}

	/// Add an inline script to this page (after existing styles)
	#[inline(always)]
	pub fn with_style_inline(mut self, style: impl Into<String>) -> Self {
		self.styles.push(ScriptSource::Inline(style.into()));
		self
	}

	/// Add a linked style to this page (after existing styles)
	#[inline(always)]
	pub fn with_style_linked(mut self, url: impl Into<String>) -> Self {
		self.styles.push(ScriptSource::Linked(url.into()));
		self
	}

	/// Add a style to this page (after existing scripts)
	#[inline(always)]
	pub fn with_style(mut self, style: ScriptSource<impl Into<String>>) -> Self {
		let style = match style {
			ScriptSource::Inline(x) => ScriptSource::Inline(x.into()),
			ScriptSource::Linked(x) => ScriptSource::Linked(x.into()),
		};

		self.scripts.push(style);
		self
	}

	/// Add a `<meta>` to this page (after existing `<meta>s`)
	#[inline(always)]
	pub fn with_extra_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
		self.extra_meta.push((key.into(), value.into()));
		self
	}
}

impl Servable for HtmlPage {
	fn head<'a>(
		&'a self,
		_ctx: &'a RenderContext,
	) -> Pin<Box<dyn Future<Output = Rendered<()>> + 'a + Send + Sync>> {
		Box::pin(async {
			return Rendered {
				code: self.response_code,
				body: (),
				ttl: self.ttl,
				private: self.private,
				headers: HeaderMap::new(),
				mime: Some(MimeType::Html),
			};
		})
	}

	fn render<'a>(
		&'a self,
		ctx: &'a RenderContext,
	) -> Pin<Box<dyn Future<Output = Rendered<RenderedBody>> + 'a + Send + Sync>> {
		Box::pin(async {
			let inner_html = (self.render)(self, ctx).await;

			let html = html! {
				(DOCTYPE)
				html {
					head {
						meta charset="UTF-8";
						meta name="viewport" content="width=device-width, initial-scale=1,user-scalable=no";
						meta content="text/html; charset=UTF-8" http-equiv="content-type";
						meta property="og:type" content="website";
						@for (name, content) in &self.extra_meta {
							meta name=(name) content=(content);
						}

						//
						// Metadata
						//
						title { (PreEscaped(self.meta.title.clone())) }
						meta property="og:site_name" content=(self.meta.title);
						meta name="title" content=(self.meta.title);
						meta property="og:title" content=(self.meta.title);
						meta property="twitter:title" content=(self.meta.title);

						@if let Some(author) = &self.meta.author {
							meta name="author" content=(author);
						}

						@if let Some(desc) = &self.meta.description {
							meta name="description" content=(desc);
							meta property="og:description" content=(desc);
							meta property="twitter:description" content=(desc);
						}

						@if let Some(image) = &self.meta.image {
							meta content=(image) property="og:image";
							link rel="shortcut icon" href=(image) type="image/x-icon";
						}

						//
						// Scripts & styles
						//

						@for style in &self.styles {
							@match style {
								ScriptSource::Linked(x) => link rel="stylesheet" type="text/css" href=(x);,
								ScriptSource::Inline(x) => style { (PreEscaped(x)) }
							}
						}

						@for script in &self.scripts {
							@match script {
								ScriptSource::Linked(x) => script src=(x) {},
								ScriptSource::Inline(x) => script { (PreEscaped(x)) }
							}
						}
					}

					body { main { (inner_html) } }
				}
			};

			return self.head(ctx).await.with_body(RenderedBody::String(html.0));
		})
	}
}
