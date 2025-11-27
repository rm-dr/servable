use axum::http::{HeaderMap, StatusCode};
use chrono::TimeDelta;
use std::collections::BTreeMap;

use crate::mime::MimeType;

//
// MARK: rendered
//

/// The contents of a response
/// produced by a [crate::servable::Servable]
#[derive(Clone)]
pub enum RenderedBody {
	/// Static raw bytes
	Static(&'static [u8]),

	/// Dynamic raw bytes
	Bytes(Vec<u8>),

	/// A UTF-8 string
	String(String),

	/// No body. Equivalent to `Self::Static(&[])`.
	Empty,
}

trait RenderedBodyTypeSealed {}
impl RenderedBodyTypeSealed for () {}
impl RenderedBodyTypeSealed for RenderedBody {}

/// A utility trait, used to control the
/// kind of body [Rendered] contains.
///
/// This trait is only implemented by two types:
/// - `()`, when a request must return an empty body (i.e, HEAD)
/// - [RenderedBody], when a request should return a full response (i.e, GET)
#[expect(private_bounds)]
pub trait RenderedBodyType: RenderedBodyTypeSealed {}
impl<T: RenderedBodyTypeSealed> RenderedBodyType for T {}

/// An asset to return from an http route
#[derive(Clone)]
pub struct Rendered<T: RenderedBodyType> {
	/// The code to return
	pub code: StatusCode,

	/// The headers to return
	pub headers: HeaderMap,

	/// The content to return
	pub body: T,

	/// The type of `self.body`
	pub mime: Option<MimeType>,

	/// How long to cache this response.
	/// If none, don't cache.
	pub ttl: Option<TimeDelta>,

	/// If true, the data at this route will never change.
	pub immutable: bool,
}

impl Rendered<()> {
	/// Turn this [Rendered] into a [Rendered] with a body.
	pub fn with_body(self, body: RenderedBody) -> Rendered<RenderedBody> {
		Rendered {
			code: self.code,
			headers: self.headers,
			body,
			mime: self.mime,
			ttl: self.ttl,
			immutable: self.immutable,
		}
	}
}

/// Additional context available to [crate::servable::Servable]s
/// when generating their content
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenderContext {
	/// Information about the request
	pub client_info: ClientInfo,

	/// The route that was requested.
	/// Starts with a /.
	pub route: String,

	/// This request's query parameters
	pub query: BTreeMap<String, String>,
}

/// The type of device that requested a page
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceType {
	/// This is a mobile device, like a phone.
	Mobile,

	/// This is a device with a large screen
	/// and a mouse, like a laptop.
	Desktop,
}

impl Default for DeviceType {
	fn default() -> Self {
		Self::Desktop
	}
}

/// Inferred information about the client
/// that requested a certain route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientInfo {
	/// The type of device that is viewing this page.
	///
	/// We do our best to detect this value automatically,
	/// but we may be wrong.
	pub device_type: DeviceType,
}

impl ClientInfo {
	pub(crate) fn from_headers(headers: &HeaderMap) -> Self {
		let ua = headers
			.get("user-agent")
			.and_then(|x| x.to_str().ok())
			.unwrap_or("");

		let ch_mobile = headers
			.get("Sec-CH-UA-Mobile")
			.and_then(|x| x.to_str().ok())
			.unwrap_or("");

		let mut device_type = None;

		if device_type.is_none() && ch_mobile.contains("1") {
			device_type = Some(DeviceType::Mobile);
		}

		if device_type.is_none() && ua.contains("Mobile") {
			device_type = Some(DeviceType::Mobile);
		}

		Self {
			device_type: device_type.unwrap_or_default(),
		}
	}
}
