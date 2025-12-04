# Servable: a simple web framework

[![CI](https://github.com/rm-dr/servable/workflows/CI/badge.svg)](https://github.com/rm-dr/servable/actions) 
[![Cargo](https://img.shields.io/crates/v/servable.svg)](https://crates.io/crates/servable) 
[![API reference](https://docs.rs/servable/badge.svg)](https://docs.rs/servable/)

A minimal, convenient web micro-framework built around [htmx](https://htmx.org), [Axum](https://github.com/tokio-rs/axum), and [Maud](https://maud.lambda.xyz). \
This powers [my homepage](https://betalupi.com). See example usage [here](https://git.betalupi.com/Mark/webpage/src/branch/main/crates/service/service-webpage/src/routes/mod.rs).

## Features

`servable` provides abstractions that implement common utilities needed by an http server.

- response headers and cache-busting utilities
- client device detection (mobile / desktop)
- server-side image optimization (see the `image` feature below)
- ergonomic [htmx](https://htmx.org) integration (see `htmx-*` features below)


-------------------


## Quick Start

```rust,ignore
use servable::{ServableRouter, servable::StaticAsset, mime::MimeType};

#[tokio::main]
async fn main() {
	let route = ServableRouter::new()
		.add_page(
			"/hello",
			StaticAsset {
				bytes: b"Hello, World!",
				mime: mime::TEXT_PLAIN
			},
		);

	// usual axum startup routine
	let app = route.into_router();
	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
		.await
		.unwrap();

	axum::serve(listener, app).await.unwrap();
}
```

# Core Concepts

## The `Servable` trait

The `Servable` trait is the foundation of this stack. \
`servable` provides implementations for a few common servables:


- `StaticAsset`, for static files like CSS, JavaScript, images, or plain bytes:
	```rust
	use servable::{StaticAsset};

	let asset = StaticAsset {
		bytes: b"body { color: red; }",
		mime: mime::TEXT_CSS,
		ttl: StaticAsset::DEFAULT_TTL
	};
	```

- `Redirect`, for simple http redirects:
	```rust
	use servable::Redirect;

	let redirect = Redirect::new("/new-location").unwrap();
	```

- `HtmlPage`, for dynamically-rendered HTML pages
	```rust
	use servable::{HtmlPage, PageMetadata};
	use maud::html;
	use std::pin::Pin;

	let page = HtmlPage::default()
		.with_meta(PageMetadata {
			title: "My Page".into(),
			description: Some("A great page".into()),
			..Default::default()
		})
		.with_render(|_page, ctx| {
			Box::pin(async move {
				html! {
					h1 { "Welcome!" }
					p { "Route: " (ctx.route) }
				}
			})
		});
	```
	`HtmlPage` automatically generates a `<head>` and wraps its rendered html in `<html><body>`.



## `ServableRouter`

A `ServableRouter` exposes a collection of `Servable`s under different routes. It implements `tower`'s `Service` trait, and can be easily be converted into an Axum `Router`. Construct one as follows:

```rust
# use servable::{ServableRouter, StaticAsset};
# let home_page = StaticAsset { bytes: b"home", mime: mime::TEXT_HTML, ttl: StaticAsset::DEFAULT_TTL};
# let about_page = StaticAsset { bytes: b"about", mime: mime::TEXT_HTML, ttl: StaticAsset::DEFAULT_TTL };
# let stylesheet = StaticAsset { bytes: b"css", mime: mime::TEXT_CSS, ttl: StaticAsset::DEFAULT_TTL };
# let custom_404_page = StaticAsset { bytes: b"404", mime: mime::TEXT_HTML, ttl: StaticAsset::DEFAULT_TTL };
let route = ServableRouter::new()
	.add_page("/", home_page)
	.add_page("/about", about_page)
	.add_page("/style.css", stylesheet)
	.with_404(custom_404_page); // override default 404
```

# Features
- `image`: enable image transformation via query parameters. This makes `tokio` a dependency. \
	  When this is enabled, all `StaticAssets` with a valid mimetype can take an optional `t=` query parameter. \
	  See the `TransformerEnum` in this crate's documentation for details.

	When `image` is enabled, the image below...
	```rust
	# use servable::{ServableRouter, StaticAsset};
	let route = ServableRouter::new()
		.add_page(
			"/image.png",
			StaticAsset {
				bytes: b"fake image data",
				mime: mime::IMAGE_PNG,
				ttl: StaticAsset::DEFAULT_TTL
			}
		);
	```
	...may be accessed as follows:

	```r
	# Original image
	GET /image.png

	# Resize to max 800px on longest side
	GET /image.png?t=maxdim(800,800)

	# Crop to a 400x400 square at the center of the image
	GET /image.png?t=crop(400,400,c)

	# Chain transformations and transcode
	GET /image.png?t=maxdim(800,800);crop(400,400);format(webp)
	```


- `htmx-2.0.8`: Include htmx sources in the compiled executable. \
	  Use as follows:
	```rust
	# use servable::ServableRouter;
	# #[cfg(feature = "htmx-2.0.8")]
	let route = ServableRouter::new()
		.add_page("/htmx.js", servable::HTMX_2_0_8)
		.add_page("/htmx-json-enc.js", servable::EXT_JSON_1_19_12);
	```



## Caching and cache-busting

Control caching behavior per servable:

```rust
use chrono::TimeDelta;
use servable::HtmlPage;

let page = HtmlPage::default()
	.with_ttl(Some(TimeDelta::hours(1)))
	.with_private(false);
```

Headers are automatically generated:
- `Cache-Control: public, max-age=3600` (default)
- `Cache-Control: private, max-age=31536000` (if `private` is true)

We also provide a static `CACHE_BUST_STR`, which may be formatted into urls to force cache refresh
whenever the server is restarted:

```rust
use chrono::TimeDelta;
use servable::{HtmlPage, CACHE_BUST_STR, ServableWithRoute, StaticAsset, ServableRouter};

pub static HTMX: ServableWithRoute<StaticAsset> = ServableWithRoute::new(
	|| format!("/{}/main.css", *CACHE_BUST_STR),
	StaticAsset {
		bytes: "div{}".as_bytes(),
		mime: mime::TEXT_CSS,
		ttl: StaticAsset::DEFAULT_TTL,
	},
);


let route = HTMX.route();
println!("Css is at {route}");

let router = ServableRouter::new()
	.add_page_with_route(&HTMX);
```

## TODO:
- cache-busting fonts in css is not possible, we need to dynamic replace urls
