use image::{DynamicImage, ImageFormat};
use mime::Mime;
use serde::{Deserialize, Deserializer, de};
use std::{fmt::Display, hash::Hash, io::Cursor, str::FromStr};
use thiserror::Error;

use super::transformers::{ImageTransformer, TransformerEnum};

#[expect(missing_docs)]
#[derive(Debug, Error)]
pub enum TransformBytesError {
	/// We tried to transform non-image data
	#[error("{0} is not a valid image type")]
	NotAnImage(String),

	/// We encountered an error while processing
	/// an image.
	#[error("error while processing image")]
	ImageError(#[from] image::ImageError),
}

/// A sequence of transformations to apply to an image
#[derive(Debug, Clone)]
pub struct TransformerChain {
	steps: Vec<TransformerEnum>,
}

impl TransformerChain {
	/// Returns `true` if `mime` is a type that can be transformed
	#[inline(always)]
	pub fn mime_is_image(mime: &Mime) -> bool {
		ImageFormat::from_mime_type(mime.to_string()).is_some()
	}

	/// Transform the given image using this chain
	#[inline(always)]
	pub fn transform_image(&self, mut image: DynamicImage) -> DynamicImage {
		for step in &self.steps {
			match step {
				TransformerEnum::Format { .. } => {}
				TransformerEnum::MaxDim(t) => t.transform(&mut image),
				TransformerEnum::Crop(t) => t.transform(&mut image),
			}
		}

		return image;
	}

	/// Return the mime this chain will produce when given an image
	/// with type `input_mime`. If this returns `None`, the input mime
	/// cannot be transformed.
	#[inline(always)]
	pub fn output_mime(&self, input_mime: &Mime) -> Option<Mime> {
		let mime = self
			.steps
			.last()
			.and_then(|x| match x {
				TransformerEnum::Format { format } => Some(
					Mime::from_str(format.to_mime_type()).unwrap_or(mime::APPLICATION_OCTET_STREAM),
				),
				_ => None,
			})
			.unwrap_or(input_mime.clone());

		let fmt = ImageFormat::from_mime_type(mime.to_string());
		fmt.map(|_| mime)
	}

	/// Transform `image_bytes` using this chain.
	/// Returns `(output_type, output_bytes)`.
	///
	/// `image_format` tells us the type of `image_bytes`.
	/// If it is `None`, we try to infer it.
	pub fn transform_bytes(
		&self,
		image_bytes: &[u8],
		image_format: Option<&Mime>,
	) -> Result<(Mime, Vec<u8>), TransformBytesError> {
		let format: ImageFormat = match image_format {
			Some(x) => ImageFormat::from_mime_type(x.to_string())
				.ok_or(TransformBytesError::NotAnImage(x.to_string()))?,
			None => image::guess_format(image_bytes)?,
		};

		let out_format = self
			.steps
			.last()
			.and_then(|x| match x {
				TransformerEnum::Format { format } => Some(format),
				_ => None,
			})
			.unwrap_or(&format);

		let img = image::load_from_memory_with_format(image_bytes, format)?;
		let img = self.transform_image(img);

		let out_mime =
			Mime::from_str(out_format.to_mime_type()).unwrap_or(mime::APPLICATION_OCTET_STREAM);
		let mut out_bytes = Cursor::new(Vec::new());
		img.write_to(&mut out_bytes, *out_format)?;

		return Ok((out_mime, out_bytes.into_inner()));
	}
}

impl FromStr for TransformerChain {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let steps_str = s.split(";");

		let mut steps = Vec::new();
		for s in steps_str {
			let s = s.trim();
			if s.is_empty() {
				continue;
			}

			let step = s.parse();
			match step {
				Ok(x) => steps.push(x),
				Err(msg) => return Err(format!("invalid step `{s}`: {msg}")),
			}
		}

		let n_format = steps
			.iter()
			.filter(|x| matches!(x, TransformerEnum::Format { .. }))
			.count();
		if n_format > 2 {
			return Err("provide at most one format()".to_owned());
		}

		if n_format == 1 && !matches!(steps.last(), Some(TransformerEnum::Format { .. })) {
			return Err("format() must be last".to_owned());
		}

		return Ok(Self { steps });
	}
}

impl<'de> Deserialize<'de> for TransformerChain {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		Self::from_str(&s).map_err(de::Error::custom)
	}
}

impl Display for TransformerChain {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut first = true;
		for step in &self.steps {
			if first {
				write!(f, "{step}")?;
				first = false
			} else {
				write!(f, ";{step}")?;
			}
		}

		return Ok(());
	}
}

impl PartialEq for TransformerChain {
	fn eq(&self, other: &Self) -> bool {
		self.to_string() == other.to_string()
	}
}

impl Eq for TransformerChain {}

impl Hash for TransformerChain {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.to_string().hash(state);
	}
}
