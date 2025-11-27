//! Strongly-typed MIME types via [MimeType].

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt::Display, str::FromStr};
use tracing::debug;

/// A media type, conveniently parsed
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MimeType {
	/// A mimetype we didn't recognize
	Other(String),

	/// An unstructured binary blob (application/octet-stream)
	Blob,

	// MARK: Audio
	/// AAC audio file (audio/aac)
	Aac,
	/// FLAC audio file (audio/flac)
	Flac,
	/// MIDI audio file (audio/midi)
	Midi,
	/// MP3 audio file (audio/mpeg)
	Mp3,
	/// OGG audio file (audio/ogg)
	Oga,
	/// Opus audio file in Ogg container (audio/ogg)
	Opus,
	/// Waveform Audio Format (audio/wav)
	Wav,
	/// WEBM audio file (audio/webm)
	Weba,

	// MARK: Video
	/// AVI: Audio Video Interleave (video/x-msvideo)
	Avi,
	/// MP4 video file (video/mp4)
	Mp4,
	/// MPEG video file (video/mpeg)
	Mpeg,
	/// OGG video file (video/ogg)
	Ogv,
	/// MPEG transport stream (video/mp2t)
	Ts,
	/// WEBM video file (video/webm)
	WebmVideo,
	/// 3GPP audio/video container (video/3gpp)
	ThreeGp,
	/// 3GPP2 audio/video container (video/3gpp2)
	ThreeG2,

	// MARK: Images
	/// Animated Portable Network Graphics (image/apng)
	Apng,
	/// AVIF image (image/avif)
	Avif,
	/// Windows OS/2 Bitmap Graphics (image/bmp)
	Bmp,
	/// Graphics Interchange Format (image/gif)
	Gif,
	/// Icon format (image/vnd.microsoft.icon)
	Ico,
	/// JPEG image (image/jpeg)
	Jpg,
	/// Portable Network Graphics (image/png)
	Png,
	/// Quite ok Image Format
	Qoi,
	/// Scalable Vector Graphics (image/svg+xml)
	Svg,
	/// Tagged Image File Format (image/tiff)
	Tiff,
	/// WEBP image (image/webp)
	Webp,

	// MARK: Text
	/// Plain text (text/plain)
	Text,
	/// Cascading Style Sheets (text/css)
	Css,
	/// Comma-separated values (text/csv)
	Csv,
	/// HyperText Markup Language (text/html)
	Html,
	/// JavaScript (text/javascript)
	Javascript,
	/// JSON format (application/json)
	Json,
	/// JSON-LD format (application/ld+json)
	JsonLd,
	/// XML (application/xml)
	Xml,

	// MARK: Documents
	/// Adobe Portable Document Format (application/pdf)
	Pdf,
	/// Rich Text Format (application/rtf)
	Rtf,

	// MARK: Archives
	/// Archive document, multiple files embedded (application/x-freearc)
	Arc,
	/// BZip archive (application/x-bzip)
	Bz,
	/// BZip2 archive (application/x-bzip2)
	Bz2,
	/// GZip Compressed Archive (application/gzip)
	Gz,
	/// Java Archive (application/java-archive)
	Jar,
	/// OGG (application/ogg)
	Ogg,
	/// RAR archive (application/vnd.rar)
	Rar,
	/// 7-zip archive (application/x-7z-compressed)
	SevenZ,
	/// Tape Archive (application/x-tar)
	Tar,
	/// ZIP archive (application/zip)
	Zip,

	// MARK: Fonts
	/// MS Embedded OpenType fonts (application/vnd.ms-fontobject)
	Eot,
	/// OpenType font (font/otf)
	Otf,
	/// TrueType Font (font/ttf)
	Ttf,
	/// Web Open Font Format (font/woff)
	Woff,
	/// Web Open Font Format 2 (font/woff2)
	Woff2,

	// MARK: Applications
	/// AbiWord document (application/x-abiword)
	Abiword,
	/// Amazon Kindle eBook format (application/vnd.amazon.ebook)
	Azw,
	/// CD audio (application/x-cdf)
	Cda,
	/// C-Shell script (application/x-csh)
	Csh,
	/// Microsoft Word (application/msword)
	Doc,
	/// Microsoft Word OpenXML (application/vnd.openxmlformats-officedocument.wordprocessingml.document)
	Docx,
	/// Electronic publication (application/epub+zip)
	Epub,
	/// iCalendar format (text/calendar)
	Ics,
	/// Apple Installer Package (application/vnd.apple.installer+xml)
	Mpkg,
	/// OpenDocument presentation (application/vnd.oasis.opendocument.presentation)
	Odp,
	/// OpenDocument spreadsheet (application/vnd.oasis.opendocument.spreadsheet)
	Ods,
	/// OpenDocument text document (application/vnd.oasis.opendocument.text)
	Odt,
	/// Hypertext Preprocessor (application/x-httpd-php)
	Php,
	/// Microsoft PowerPoint (application/vnd.ms-powerpoint)
	Ppt,
	/// Microsoft PowerPoint OpenXML (application/vnd.openxmlformats-officedocument.presentationml.presentation)
	Pptx,
	/// Bourne shell script (application/x-sh)
	Sh,
	/// Microsoft Visio (application/vnd.visio)
	Vsd,
	/// XHTML (application/xhtml+xml)
	Xhtml,
	/// Microsoft Excel (application/vnd.ms-excel)
	Xls,
	/// Microsoft Excel OpenXML (application/vnd.openxmlformats-officedocument.spreadsheetml.sheet)
	Xlsx,
	/// XUL (application/vnd.mozilla.xul+xml)
	Xul,
}

// MARK: ser/de

/*
impl utoipa::ToSchema for MimeType {
	fn name() -> std::borrow::Cow<'static, str> {
		std::borrow::Cow::Borrowed("MimeType")
	}
}
impl utoipa::PartialSchema for MimeType {
	fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
		utoipa::openapi::Schema::Object(
			utoipa::openapi::schema::ObjectBuilder::new()
				.schema_type(utoipa::openapi::schema::SchemaType::Type(Type::String))
				.description(Some(
					"A media type string (e.g., 'application/json', 'text/plain')",
				))
				.examples(Some("application/json"))
				.build(),
		)
		.into()
	}
}
*/

impl Serialize for MimeType {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

impl<'de> Deserialize<'de> for MimeType {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		Ok(MimeType::from_str(&s).unwrap())
	}
}

//
// MARK: misc
//

impl Default for MimeType {
	fn default() -> Self {
		Self::Blob
	}
}

impl From<String> for MimeType {
	fn from(value: String) -> Self {
		Self::from_str(&value).unwrap()
	}
}

impl From<&str> for MimeType {
	fn from(value: &str) -> Self {
		Self::from_str(value).unwrap()
	}
}

impl From<&MimeType> for String {
	fn from(value: &MimeType) -> Self {
		value.to_string()
	}
}

//
// MARK: fromstr
//

impl MimeType {
	/// Parse a mimetype from a string that may contain
	/// whitespace or ";" parameters.
	///
	/// Parameters are discarded, write your own parser if you need them.
	pub fn from_header(s: &str) -> Result<Self, <Self as FromStr>::Err> {
		let s = s.trim();
		let semi = s.find(';').unwrap_or(s.len());
		let space = s.find(' ').unwrap_or(s.len());
		let limit = semi.min(space);
		let s = &s[0..limit];
		let s = s.trim();

		return Self::from_str(s);
	}
}

impl FromStr for MimeType {
	type Err = std::convert::Infallible;

	// Must match `display` below, but may provide other alternatives.
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"application/octet-stream" => Self::Blob,

			// Audio
			"audio/aac" => Self::Aac,
			"audio/flac" => Self::Flac,
			"audio/midi" | "audio/x-midi" => Self::Midi,
			"audio/mpeg" => Self::Mp3,
			"audio/ogg" => Self::Oga,
			"audio/wav" => Self::Wav,
			"audio/webm" => Self::Weba,

			// Video
			"video/x-msvideo" => Self::Avi,
			"video/mp4" => Self::Mp4,
			"video/mpeg" => Self::Mpeg,
			"video/ogg" => Self::Ogv,
			"video/mp2t" => Self::Ts,
			"video/webm" => Self::WebmVideo,
			"video/3gpp" => Self::ThreeGp,
			"video/3gpp2" => Self::ThreeG2,

			// Images
			"image/apng" => Self::Apng,
			"image/avif" => Self::Avif,
			"image/bmp" => Self::Bmp,
			"image/gif" => Self::Gif,
			"image/vnd.microsoft.icon" => Self::Ico,
			"image/jpeg" | "image/jpg" => Self::Jpg,
			"image/png" => Self::Png,
			"image/svg+xml" => Self::Svg,
			"image/tiff" => Self::Tiff,
			"image/webp" => Self::Webp,
			"image/qoi" => Self::Qoi,

			// Text
			"text/plain" => Self::Text,
			"text/css" => Self::Css,
			"text/csv" => Self::Csv,
			"text/html" => Self::Html,
			"text/javascript" => Self::Javascript,
			"application/json" => Self::Json,
			"application/ld+json" => Self::JsonLd,
			"application/xml" | "text/xml" => Self::Xml,

			// Documents
			"application/pdf" => Self::Pdf,
			"application/rtf" => Self::Rtf,

			// Archives
			"application/x-freearc" => Self::Arc,
			"application/x-bzip" => Self::Bz,
			"application/x-bzip2" => Self::Bz2,
			"application/gzip" | "application/x-gzip" => Self::Gz,
			"application/java-archive" => Self::Jar,
			"application/ogg" => Self::Ogg,
			"application/vnd.rar" => Self::Rar,
			"application/x-7z-compressed" => Self::SevenZ,
			"application/x-tar" => Self::Tar,
			"application/zip" | "application/x-zip-compressed" => Self::Zip,

			// Fonts
			"application/vnd.ms-fontobject" => Self::Eot,
			"font/otf" => Self::Otf,
			"font/ttf" => Self::Ttf,
			"font/woff" => Self::Woff,
			"font/woff2" => Self::Woff2,

			// Applications
			"application/x-abiword" => Self::Abiword,
			"application/vnd.amazon.ebook" => Self::Azw,
			"application/x-cdf" => Self::Cda,
			"application/x-csh" => Self::Csh,
			"application/msword" => Self::Doc,
			"application/vnd.openxmlformats-officedocument.wordprocessingml.document" => Self::Docx,
			"application/epub+zip" => Self::Epub,
			"text/calendar" => Self::Ics,
			"application/vnd.apple.installer+xml" => Self::Mpkg,
			"application/vnd.oasis.opendocument.presentation" => Self::Odp,
			"application/vnd.oasis.opendocument.spreadsheet" => Self::Ods,
			"application/vnd.oasis.opendocument.text" => Self::Odt,
			"application/x-httpd-php" => Self::Php,
			"application/vnd.ms-powerpoint" => Self::Ppt,
			"application/vnd.openxmlformats-officedocument.presentationml.presentation" => {
				Self::Pptx
			}
			"application/x-sh" => Self::Sh,
			"application/vnd.visio" => Self::Vsd,
			"application/xhtml+xml" => Self::Xhtml,
			"application/vnd.ms-excel" => Self::Xls,
			"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => Self::Xlsx,
			"application/vnd.mozilla.xul+xml" => Self::Xul,

			_ => {
				debug!(message = "Encountered unknown mimetype", mime_string = s);
				Self::Other(s.into())
			}
		})
	}
}

//
// MARK: display
//

impl Display for MimeType {
	/// Get a string representation of this mimetype.
	///
	/// The following always holds:
	/// ```rust
	/// # use servable::mime::MimeType;
	/// # let x = MimeType::Blob;
	/// assert_eq!(MimeType::from(x.to_string()), x);
	/// ```
	///
	/// The following might not hold:
	/// ```rust
	/// # use servable::mime::MimeType;
	/// # let y = "application/custom";
	/// // MimeType::from(y).to_string() may not equal y
	/// ```
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Blob => write!(f, "application/octet-stream"),

			// Audio
			Self::Aac => write!(f, "audio/aac"),
			Self::Flac => write!(f, "audio/flac"),
			Self::Midi => write!(f, "audio/midi"),
			Self::Mp3 => write!(f, "audio/mpeg"),
			Self::Oga => write!(f, "audio/ogg"),
			Self::Opus => write!(f, "audio/ogg"),
			Self::Wav => write!(f, "audio/wav"),
			Self::Weba => write!(f, "audio/webm"),

			// Video
			Self::Avi => write!(f, "video/x-msvideo"),
			Self::Mp4 => write!(f, "video/mp4"),
			Self::Mpeg => write!(f, "video/mpeg"),
			Self::Ogv => write!(f, "video/ogg"),
			Self::Ts => write!(f, "video/mp2t"),
			Self::WebmVideo => write!(f, "video/webm"),
			Self::ThreeGp => write!(f, "video/3gpp"),
			Self::ThreeG2 => write!(f, "video/3gpp2"),

			// Images
			Self::Apng => write!(f, "image/apng"),
			Self::Avif => write!(f, "image/avif"),
			Self::Bmp => write!(f, "image/bmp"),
			Self::Gif => write!(f, "image/gif"),
			Self::Ico => write!(f, "image/vnd.microsoft.icon"),
			Self::Jpg => write!(f, "image/jpeg"),
			Self::Png => write!(f, "image/png"),
			Self::Svg => write!(f, "image/svg+xml"),
			Self::Tiff => write!(f, "image/tiff"),
			Self::Webp => write!(f, "image/webp"),
			Self::Qoi => write!(f, "image/qoi"),

			// Text
			Self::Text => write!(f, "text/plain"),
			Self::Css => write!(f, "text/css"),
			Self::Csv => write!(f, "text/csv"),
			Self::Html => write!(f, "text/html"),
			Self::Javascript => write!(f, "text/javascript"),
			Self::Json => write!(f, "application/json"),
			Self::JsonLd => write!(f, "application/ld+json"),
			Self::Xml => write!(f, "application/xml"),

			// Documents
			Self::Pdf => write!(f, "application/pdf"),
			Self::Rtf => write!(f, "application/rtf"),

			// Archives
			Self::Arc => write!(f, "application/x-freearc"),
			Self::Bz => write!(f, "application/x-bzip"),
			Self::Bz2 => write!(f, "application/x-bzip2"),
			Self::Gz => write!(f, "application/gzip"),
			Self::Jar => write!(f, "application/java-archive"),
			Self::Ogg => write!(f, "application/ogg"),
			Self::Rar => write!(f, "application/vnd.rar"),
			Self::SevenZ => write!(f, "application/x-7z-compressed"),
			Self::Tar => write!(f, "application/x-tar"),
			Self::Zip => write!(f, "application/zip"),

			// Fonts
			Self::Eot => write!(f, "application/vnd.ms-fontobject"),
			Self::Otf => write!(f, "font/otf"),
			Self::Ttf => write!(f, "font/ttf"),
			Self::Woff => write!(f, "font/woff"),
			Self::Woff2 => write!(f, "font/woff2"),

			// Applications
			Self::Abiword => write!(f, "application/x-abiword"),
			Self::Azw => write!(f, "application/vnd.amazon.ebook"),
			Self::Cda => write!(f, "application/x-cdf"),
			Self::Csh => write!(f, "application/x-csh"),
			Self::Doc => write!(f, "application/msword"),
			Self::Docx => write!(
				f,
				"application/vnd.openxmlformats-officedocument.wordprocessingml.document"
			),
			Self::Epub => write!(f, "application/epub+zip"),
			Self::Ics => write!(f, "text/calendar"),
			Self::Mpkg => write!(f, "application/vnd.apple.installer+xml"),
			Self::Odp => write!(f, "application/vnd.oasis.opendocument.presentation"),
			Self::Ods => write!(f, "application/vnd.oasis.opendocument.spreadsheet"),
			Self::Odt => write!(f, "application/vnd.oasis.opendocument.text"),
			Self::Php => write!(f, "application/x-httpd-php"),
			Self::Ppt => write!(f, "application/vnd.ms-powerpoint"),
			Self::Pptx => write!(
				f,
				"application/vnd.openxmlformats-officedocument.presentationml.presentation"
			),
			Self::Sh => write!(f, "application/x-sh"),
			Self::Vsd => write!(f, "application/vnd.visio"),
			Self::Xhtml => write!(f, "application/xhtml+xml"),
			Self::Xls => write!(f, "application/vnd.ms-excel"),
			Self::Xlsx => write!(
				f,
				"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
			),
			Self::Xul => write!(f, "application/vnd.mozilla.xul+xml"),

			Self::Other(x) => write!(f, "{x}"),
		}
	}
}

impl MimeType {
	//
	// MARK: from extension
	//

	/// Try to guess a file's mime type from its extension.
	/// `ext` should NOT start with a dot.
	pub fn from_extension(ext: &str) -> Option<Self> {
		Some(match ext {
			// Audio
			"aac" => Self::Aac,
			"flac" => Self::Flac,
			"mid" | "midi" => Self::Midi,
			"mp3" => Self::Mp3,
			"oga" => Self::Oga,
			"opus" => Self::Opus,
			"wav" => Self::Wav,
			"weba" => Self::Weba,

			// Video
			"avi" => Self::Avi,
			"mp4" => Self::Mp4,
			"mpeg" => Self::Mpeg,
			"ogv" => Self::Ogv,
			"ts" => Self::Ts,
			"webm" => Self::WebmVideo,
			"3gp" => Self::ThreeGp,
			"3g2" => Self::ThreeG2,

			// Images
			"apng" => Self::Apng,
			"avif" => Self::Avif,
			"bmp" => Self::Bmp,
			"gif" => Self::Gif,
			"ico" => Self::Ico,
			"jpg" | "jpeg" => Self::Jpg,
			"png" => Self::Png,
			"svg" => Self::Svg,
			"tif" | "tiff" => Self::Tiff,
			"webp" => Self::Webp,
			"qoi" => Self::Qoi,

			// Text
			"txt" => Self::Text,
			"css" => Self::Css,
			"csv" => Self::Csv,
			"htm" | "html" => Self::Html,
			"js" | "mjs" => Self::Javascript,
			"json" => Self::Json,
			"jsonld" => Self::JsonLd,
			"xml" => Self::Xml,

			// Documents
			"pdf" => Self::Pdf,
			"rtf" => Self::Rtf,

			// Archives
			"arc" => Self::Arc,
			"bz" => Self::Bz,
			"bz2" => Self::Bz2,
			"gz" => Self::Gz,
			"jar" => Self::Jar,
			"ogx" => Self::Ogg,
			"rar" => Self::Rar,
			"7z" => Self::SevenZ,
			"tar" => Self::Tar,
			"zip" => Self::Zip,

			// Fonts
			"eot" => Self::Eot,
			"otf" => Self::Otf,
			"ttf" => Self::Ttf,
			"woff" => Self::Woff,
			"woff2" => Self::Woff2,

			// Applications
			"abw" => Self::Abiword,
			"azw" => Self::Azw,
			"cda" => Self::Cda,
			"csh" => Self::Csh,
			"doc" => Self::Doc,
			"docx" => Self::Docx,
			"epub" => Self::Epub,
			"ics" => Self::Ics,
			"mpkg" => Self::Mpkg,
			"odp" => Self::Odp,
			"ods" => Self::Ods,
			"odt" => Self::Odt,
			"php" => Self::Php,
			"ppt" => Self::Ppt,
			"pptx" => Self::Pptx,
			"sh" => Self::Sh,
			"vsd" => Self::Vsd,
			"xhtml" => Self::Xhtml,
			"xls" => Self::Xls,
			"xlsx" => Self::Xlsx,
			"xul" => Self::Xul,

			_ => return None,
		})
	}

	//
	// MARK: to extension
	//

	/// Get the extension we use for files with this type.
	/// Never includes a dot.
	pub fn extension(&self) -> Option<&'static str> {
		match self {
			Self::Blob => None,
			Self::Other(_) => None,

			// Audio
			Self::Aac => Some("aac"),
			Self::Flac => Some("flac"),
			Self::Midi => Some("midi"),
			Self::Mp3 => Some("mp3"),
			Self::Oga => Some("oga"),
			Self::Opus => Some("opus"),
			Self::Wav => Some("wav"),
			Self::Weba => Some("weba"),

			// Video
			Self::Avi => Some("avi"),
			Self::Mp4 => Some("mp4"),
			Self::Mpeg => Some("mpeg"),
			Self::Ogv => Some("ogv"),
			Self::Ts => Some("ts"),
			Self::WebmVideo => Some("webm"),
			Self::ThreeGp => Some("3gp"),
			Self::ThreeG2 => Some("3g2"),

			// Images
			Self::Apng => Some("apng"),
			Self::Avif => Some("avif"),
			Self::Bmp => Some("bmp"),
			Self::Gif => Some("gif"),
			Self::Ico => Some("ico"),
			Self::Jpg => Some("jpg"),
			Self::Png => Some("png"),
			Self::Svg => Some("svg"),
			Self::Tiff => Some("tiff"),
			Self::Webp => Some("webp"),
			Self::Qoi => Some("qoi"),

			// Text
			Self::Text => Some("txt"),
			Self::Css => Some("css"),
			Self::Csv => Some("csv"),
			Self::Html => Some("html"),
			Self::Javascript => Some("js"),
			Self::Json => Some("json"),
			Self::JsonLd => Some("jsonld"),
			Self::Xml => Some("xml"),

			// Documents
			Self::Pdf => Some("pdf"),
			Self::Rtf => Some("rtf"),

			// Archives
			Self::Arc => Some("arc"),
			Self::Bz => Some("bz"),
			Self::Bz2 => Some("bz2"),
			Self::Gz => Some("gz"),
			Self::Jar => Some("jar"),
			Self::Ogg => Some("ogx"),
			Self::Rar => Some("rar"),
			Self::SevenZ => Some("7z"),
			Self::Tar => Some("tar"),
			Self::Zip => Some("zip"),

			// Fonts
			Self::Eot => Some("eot"),
			Self::Otf => Some("otf"),
			Self::Ttf => Some("ttf"),
			Self::Woff => Some("woff"),
			Self::Woff2 => Some("woff2"),

			// Applications
			Self::Abiword => Some("abw"),
			Self::Azw => Some("azw"),
			Self::Cda => Some("cda"),
			Self::Csh => Some("csh"),
			Self::Doc => Some("doc"),
			Self::Docx => Some("docx"),
			Self::Epub => Some("epub"),
			Self::Ics => Some("ics"),
			Self::Mpkg => Some("mpkg"),
			Self::Odp => Some("odp"),
			Self::Ods => Some("ods"),
			Self::Odt => Some("odt"),
			Self::Php => Some("php"),
			Self::Ppt => Some("ppt"),
			Self::Pptx => Some("pptx"),
			Self::Sh => Some("sh"),
			Self::Vsd => Some("vsd"),
			Self::Xhtml => Some("xhtml"),
			Self::Xls => Some("xls"),
			Self::Xlsx => Some("xlsx"),
			Self::Xul => Some("xul"),
		}
	}

	//
	// MARK: is_text
	//

	/// Returns true if this MIME type is always plain text.
	pub fn is_text(&self) -> bool {
		match self {
			// Text types
			Self::Text => true,
			Self::Css => true,
			Self::Csv => true,
			Self::Html => true,
			Self::Javascript => true,
			Self::Json => true,
			Self::JsonLd => true,
			Self::Xml => true,
			Self::Svg => true,
			Self::Ics => true,
			Self::Xhtml => true,

			// Script types
			Self::Csh => true,
			Self::Php => true,
			Self::Sh => true,

			// All other types are not plain text
			Self::Other(_) => false,
			Self::Blob => false,

			// Audio
			Self::Aac => false,
			Self::Flac => false,
			Self::Midi => false,
			Self::Mp3 => false,
			Self::Oga => false,
			Self::Opus => false,
			Self::Wav => false,
			Self::Weba => false,

			// Video
			Self::Avi => false,
			Self::Mp4 => false,
			Self::Mpeg => false,
			Self::Ogv => false,
			Self::Ts => false,
			Self::WebmVideo => false,
			Self::ThreeGp => false,
			Self::ThreeG2 => false,

			// Images
			Self::Apng => false,
			Self::Avif => false,
			Self::Bmp => false,
			Self::Gif => false,
			Self::Ico => false,
			Self::Jpg => false,
			Self::Png => false,
			Self::Qoi => false,
			Self::Tiff => false,
			Self::Webp => false,

			// Documents
			Self::Pdf => false,
			Self::Rtf => false,

			// Archives
			Self::Arc => false,
			Self::Bz => false,
			Self::Bz2 => false,
			Self::Gz => false,
			Self::Jar => false,
			Self::Ogg => false,
			Self::Rar => false,
			Self::SevenZ => false,
			Self::Tar => false,
			Self::Zip => false,

			// Fonts
			Self::Eot => false,
			Self::Otf => false,
			Self::Ttf => false,
			Self::Woff => false,
			Self::Woff2 => false,

			// Applications
			Self::Abiword => false,
			Self::Azw => false,
			Self::Cda => false,
			Self::Doc => false,
			Self::Docx => false,
			Self::Epub => false,
			Self::Mpkg => false,
			Self::Odp => false,
			Self::Ods => false,
			Self::Odt => false,
			Self::Ppt => false,
			Self::Pptx => false,
			Self::Vsd => false,
			Self::Xls => false,
			Self::Xlsx => false,
			Self::Xul => false,
		}
	}
}
