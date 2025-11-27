use image::{DynamicImage, imageops::FilterType};
use std::fmt::Display;

use super::super::{pixeldim::PixelDim, transformers::ImageTransformer};

/// Scale an image until it fits in a configured bounding box.
#[derive(Debug, Clone, PartialEq)]
pub struct MaxDimTransformer {
	w: PixelDim,
	h: PixelDim,
}

impl MaxDimTransformer {
	/// Create a new [MaxDimTransformer] that scales an image down
	/// until it fits in a box of dimension `w x h`.
	///
	/// Images are never scaled up.
	pub fn new(w: PixelDim, h: PixelDim) -> Self {
		Self { w, h }
	}

	fn target_dim(&self, img_width: u32, img_height: u32) -> (u32, u32) {
		let max_width = match self.w {
			PixelDim::Pixels(w) => Some(w),
			PixelDim::WidthPercent(pct) => Some(((img_width as f32) * pct / 100.0) as u32),
			PixelDim::HeightPercent(_) => None,
		};

		let max_height = match self.h {
			PixelDim::Pixels(h) => Some(h),
			PixelDim::HeightPercent(pct) => Some(((img_height as f32) * pct / 100.0) as u32),
			PixelDim::WidthPercent(_) => None,
		};

		if max_width.map(|x| img_width <= x).unwrap_or(true)
			&& max_height.map(|x| img_height <= x).unwrap_or(true)
		{
			return (img_width, img_height);
		}

		let width_ratio = max_width
			.map(|x| x as f32 / img_width as f32)
			.unwrap_or(1.0);

		let height_ratio = max_height
			.map(|x| x as f32 / img_height as f32)
			.unwrap_or(1.0);

		let ratio = width_ratio.min(height_ratio);

		(
			(img_width as f32 * ratio) as u32,
			(img_height as f32 * ratio) as u32,
		)
	}
}

impl Display for MaxDimTransformer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "maxdim({},{})", self.w, self.h)
	}
}

impl ImageTransformer for MaxDimTransformer {
	fn parse_args(args: &str) -> Result<Self, String> {
		let args: Vec<&str> = args.split(",").collect();
		if args.len() != 2 {
			return Err(format!("expected 2 args, got {}", args.len()));
		}

		let w = args[0].parse::<PixelDim>()?;
		let h = args[1].parse::<PixelDim>()?;

		Ok(Self { w, h })
	}

	fn transform(&self, input: &mut DynamicImage) {
		let (img_width, img_height) = (input.width(), input.height());
		let (target_width, target_height) = self.target_dim(img_width, img_height);

		// Only resize if needed
		if target_width != img_width || target_height != img_height {
			*input = input.resize(target_width, target_height, FilterType::Lanczos3);
		}
	}
}
