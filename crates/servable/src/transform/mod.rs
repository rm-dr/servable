//! Provides simple server-side image optimization
//! using query parameters.

mod pixeldim;

pub mod transformers;

mod chain;
pub use chain::*;
