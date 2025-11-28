#![doc = include_str!("../README.md")]
// readme is symlinked to the root of this repo
// because `cargo publish` works from a different dir,
// and needs a different relative path than cargo build.
// https://github.com/rust-lang/cargo/issues/13309

pub mod mime;

mod types;

use rand::{Rng, distr::Alphanumeric};
pub use types::*;

mod router;
pub use router::*;

mod servable;
pub use servable::*;

#[cfg(test)] // Used in doctests
use tower_http as _;

//
//
//

#[cfg(feature = "image")]
pub mod transform;

/// A unique string that can be used for cache-busting.
///
/// Note that this string changes every time this code is started,
/// even if the data inside the program did not change.
pub static CACHE_BUST_STR: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
	rand::rng()
		.sample_iter(&Alphanumeric)
		.take(10)
		.map(char::from)
		.collect()
});

//
//
//

/// HTMX 2.0.8, minified
#[cfg(feature = "htmx-2.0.8")]
pub const HTMX_2_0_8: servable::StaticAsset = servable::StaticAsset {
	bytes: include_str!("../htmx/htmx-2.0.8.min.js").as_bytes(),
	mime: mime::MimeType::Javascript,
	ttl: StaticAsset::DEFAULT_TTL,
};

/// HTMX json extension, 1.19.2.
/// Compatible with:
/// - [HTMX_2_0_8]
#[cfg(feature = "htmx-2.0.8")]
pub const EXT_JSON_1_19_12: servable::StaticAsset = servable::StaticAsset {
	bytes: include_str!("../htmx/json-enc-1.9.12.js").as_bytes(),
	mime: mime::MimeType::Javascript,
	ttl: StaticAsset::DEFAULT_TTL,
};
