

use super::{ColorSpace, RgbEncoding};
use std::fmt;


#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone, Copy, Default)]
pub struct Rgb {
	r: u8,
	g: u8,
	b: u8,
}

impl Into<RgbEncoding> for Rgb {
	fn into(self) -> RgbEncoding {
		RgbEncoding {r: self.r, g: self.g, b: self.b}
	}
}

impl From<RgbEncoding> for Rgb {
    fn from(rgb: RgbEncoding) -> Self {
    	Rgb {r: rgb.r, g: rgb.g, b: rgb.b}
    }
}

impl ColorSpace for Rgb {}


impl fmt::Display for Rgb {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:?}", self)
	}
}

impl fmt::UpperHex for Rgb {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
	}
}

impl fmt::LowerHex for Rgb {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
	}
}