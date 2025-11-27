//! Defines all transformation steps we can apply to an image

use image::{DynamicImage, ImageFormat};
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

mod crop;
pub use crop::*;

mod maxdim;
pub use maxdim::*;

/// A single transformation that may be applied to an image.
pub trait ImageTransformer
where
	Self: PartialEq,
	Self: Sized + Clone,
	Self: Display + Debug,
{
	/// Transform the given image in place
	fn transform(&self, input: &mut DynamicImage);

	/// Parse an arg string.
	///
	/// `name({arg_string})`
	fn parse_args(args: &str) -> Result<Self, String>;
}

use serde::{Deserialize, Deserializer};

/// An enum of all [`ImageTransformer`]s
#[derive(Debug, Clone, PartialEq)]
pub enum TransformerEnum {
	/// Usage: `maxdim(w, h)`
	///
	/// Scale the image so its width is smaller than `w`
	/// and its height is smaller than `h`. Aspect ratio is preserved.
	///
	/// To only limit the size of one dimension, use `vw` or `vh`.
	/// For example, `maxdim(50,100vh)` will not limit width.
	MaxDim(MaxDimTransformer),

	/// Usage: `crop(w, h, float)`
	///
	/// Crop the image to at most `w` by `h` pixels,
	/// floating the crop area in the specified direction.
	///
	/// Directions are one of:
	/// - Cardinal: n,e,s,w
	/// - Diagonal: ne,nw,se,sw,
	/// - Centered: c
	///
	/// Examples:
	/// - `crop(100vw, 50)` gets the top 50 pixels of the image \
	///   (or fewer, if the image's height is smaller than 50)
	///
	/// To only limit the size of one dimension, use `vw` or `vh`.
	/// For example, `maxdim(50,100vh)` will not limit width.
	Crop(CropTransformer),

	/// Usage: `format(format)`
	///
	/// Transcode the image to the given format.
	/// This step must be last, and cannot be provided
	/// more than once.
	///
	/// Valid formats:
	/// - bmp
	/// - gif
	/// - ico
	/// - jpeg or jpg
	/// - png
	/// - qoi
	/// - webp
	///
	/// Example:
	/// - `format(png)`
	///
	/// When transcoding an animated gif, the first frame is taken
	/// and all others are thrown away. This happens even if we
	/// transcode from a gif to a gif.
	Format {
		/// The format to produce
		format: ImageFormat,
	},
}

impl FromStr for TransformerEnum {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = s.trim();

		let (name, args) = {
			let name_len = match s.find('(') {
				Some(x) => x + 1,
				None => {
					return Err(format!(
						"invalid transformation {s}. Must look like name(args)."
					));
				}
			};

			let mut balance = 1;
			let mut end = name_len;
			for i in s[name_len..].bytes() {
				match i {
					b')' => balance -= 1,
					b'(' => balance += 1,
					_ => {}
				}

				if balance == 0 {
					break;
				}

				end += 1;
			}

			if balance != 0 {
				return Err(format!("mismatched parenthesis in {s}"));
			}

			let name = s[0..name_len - 1].trim();
			let args = s[name_len..end].trim();
			let trail = s[end + 1..].trim();
			if !trail.is_empty() {
				return Err(format!(
					"invalid transformation {s}. Must look like name(args)."
				));
			}

			(name, args)
		};

		match name {
			"maxdim" => Ok(Self::MaxDim(MaxDimTransformer::parse_args(args)?)),
			"crop" => Ok(Self::Crop(CropTransformer::parse_args(args)?)),

			"format" => Ok(TransformerEnum::Format {
				format: ImageFormat::from_extension(args)
					.ok_or(format!("invalid image format {args}"))?,
			}),

			_ => Err(format!("unknown transformation {name}")),
		}
	}
}

impl<'de> Deserialize<'de> for TransformerEnum {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		s.parse().map_err(serde::de::Error::custom)
	}
}

impl Display for TransformerEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			TransformerEnum::MaxDim(x) => Display::fmt(x, f),
			TransformerEnum::Crop(x) => Display::fmt(x, f),
			TransformerEnum::Format { format } => {
				write!(f, "format({})", format.extensions_str()[0])
			}
		}
	}
}
