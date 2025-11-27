use serde::{Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;

// TODO: parse -, + (100vw - 10px)
// TODO: parse 100vw [min] 10
// TODO: parse 100vw [max] 10

#[derive(Debug, Clone, PartialEq)]
pub enum PixelDim {
	Pixels(u32),
	WidthPercent(f32),
	HeightPercent(f32),
}

impl FromStr for PixelDim {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let numeric_end = s.find(|c: char| !c.is_ascii_digit() && c != '.');

		let (quantity, unit) = numeric_end.map(|x| s.split_at(x)).unwrap_or((s, "px"));
		let quantity = quantity.trim();
		let unit = unit.trim();

		match unit {
			"vw" => Ok(PixelDim::WidthPercent(
				quantity
					.parse()
					.map_err(|_err| format!("invalid quantity {quantity}"))?,
			)),

			"vh" => Ok(PixelDim::HeightPercent(
				quantity
					.parse()
					.map_err(|_err| format!("invalid quantity {quantity}"))?,
			)),

			"px" => Ok(PixelDim::Pixels(
				quantity
					.parse()
					.map_err(|_err| format!("invalid quantity {quantity}"))?,
			)),

			_ => Err(format!("invalid unit {unit}")),
		}
	}
}

impl<'de> Deserialize<'de> for PixelDim {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		FromStr::from_str(&s).map_err(serde::de::Error::custom)
	}
}

impl fmt::Display for PixelDim {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			PixelDim::Pixels(px) => write!(f, "{px}"),
			PixelDim::WidthPercent(p) => write!(f, "{p:.2}vw"),
			PixelDim::HeightPercent(p) => write!(f, "{p:.2}vh"),
		}
	}
}
