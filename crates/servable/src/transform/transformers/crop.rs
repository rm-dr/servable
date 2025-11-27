use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use strum::{Display, EnumString};

use super::super::{pixeldim::PixelDim, transformers::ImageTransformer};

#[expect(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Serialize, Deserialize, Display)]
pub enum Direction {
	#[serde(rename = "n")]
	#[strum(to_string = "n")]
	#[strum(serialize = "north")]
	North,

	#[serde(rename = "e")]
	#[strum(serialize = "e")]
	#[strum(serialize = "east")]
	East,

	#[serde(rename = "s")]
	#[strum(serialize = "s")]
	#[strum(serialize = "south")]
	South,

	#[serde(rename = "w")]
	#[strum(to_string = "w")]
	#[strum(serialize = "west")]
	West,

	#[serde(rename = "c")]
	#[strum(serialize = "c")]
	#[strum(serialize = "center")]
	Center,

	#[serde(rename = "ne")]
	#[strum(serialize = "ne")]
	#[strum(serialize = "northeast")]
	NorthEast,

	#[serde(rename = "se")]
	#[strum(serialize = "se")]
	#[strum(serialize = "southeast")]
	SouthEast,

	#[serde(rename = "nw")]
	#[strum(serialize = "nw")]
	#[strum(serialize = "northwest")]
	NorthWest,

	#[serde(rename = "sw")]
	#[strum(serialize = "sw")]
	#[strum(serialize = "southwest")]
	SouthWest,
}

/// Crop an image to (at most) the given size.
/// See [Self::new] for details.
#[derive(Debug, Clone, PartialEq)]
pub struct CropTransformer {
	w: PixelDim,
	h: PixelDim,
	float: Direction,
}

impl CropTransformer {
	/// Create a new [CropTransformer] with the given parameters.
	///
	/// A [CropTransformer] creates an image of size `w x h`, but...
	/// - does not reduce width if `w` is greater than image width
	/// - does not reduce height if `h` is greater than image height
	/// - does nothing if `w` or `h` is less than or equal to zero.
	pub fn new(w: PixelDim, h: PixelDim, float: Direction) -> Self {
		Self { w, h, float }
	}

	fn crop_dim(&self, img_width: u32, img_height: u32) -> (u32, u32) {
		let crop_width = match self.w {
			PixelDim::Pixels(w) => w,
			PixelDim::WidthPercent(pct) => ((img_width as f32) * pct / 100.0) as u32,
			PixelDim::HeightPercent(pct) => ((img_height as f32) * pct / 100.0) as u32,
		};

		let crop_height = match self.h {
			PixelDim::Pixels(h) => h,
			PixelDim::WidthPercent(pct) => ((img_width as f32) * pct / 100.0) as u32,
			PixelDim::HeightPercent(pct) => ((img_height as f32) * pct / 100.0) as u32,
		};

		(crop_width, crop_height)
	}

	#[expect(clippy::integer_division)]
	fn crop_pos(
		&self,
		img_width: u32,
		img_height: u32,
		crop_width: u32,
		crop_height: u32,
	) -> (u32, u32) {
		match self.float {
			Direction::North => {
				let x = (img_width - crop_width) / 2;
				let y = 0;
				(x, y)
			}
			Direction::East => {
				let x = img_width - crop_width;
				let y = (img_height - crop_height) / 2;
				(x, y)
			}
			Direction::South => {
				let x = (img_width - crop_width) / 2;
				let y = img_height - crop_height;
				(x, y)
			}
			Direction::West => {
				let x = 0;
				let y = (img_height - crop_height) / 2;
				(x, y)
			}
			Direction::Center => {
				let x = (img_width - crop_width) / 2;
				let y = (img_height - crop_height) / 2;
				(x, y)
			}
			Direction::NorthEast => {
				let x = img_width - crop_width;
				let y = 0;
				(x, y)
			}
			Direction::SouthEast => {
				let x = img_width - crop_width;
				let y = img_height - crop_height;
				(x, y)
			}
			Direction::NorthWest => {
				let x = 0;
				let y = 0;
				(x, y)
			}
			Direction::SouthWest => {
				let x = 0;
				let y = img_height - crop_height;
				(x, y)
			}
		}
	}
}

impl Display for CropTransformer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "crop({},{},{})", self.w, self.h, self.float)
	}
}

impl ImageTransformer for CropTransformer {
	fn parse_args(args: &str) -> Result<Self, String> {
		let args: Vec<&str> = args.split(",").collect();
		if args.len() != 3 {
			return Err(format!("expected 3 args, got {}", args.len()));
		}

		let w = args[0].trim().parse::<PixelDim>()?;
		let h = args[1].trim().parse::<PixelDim>()?;

		let direction = args[2].trim();
		let direction = Direction::from_str(direction)
			.map_err(|_err| format!("invalid direction {direction}"))?;

		Ok(Self {
			w,
			h,
			float: direction,
		})
	}

	fn transform(&self, input: &mut DynamicImage) {
		let (img_width, img_height) = (input.width(), input.height());
		let (crop_width, crop_height) = self.crop_dim(img_width, img_height);

		if (crop_width < img_width || crop_height < img_height) && crop_width > 0 && crop_height > 0
		{
			let (x, y) = self.crop_pos(img_width, img_height, crop_width, crop_height);
			*input = input.crop(x, y, crop_width, crop_height);
		}
	}
}
