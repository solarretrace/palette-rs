// The MIT License (MIT)
// 
// Copyright (c) 2016 Skylor R. Schermer
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in 
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
////////////////////////////////////////////////////////////////////////////////
//!
//! Defines a 32-bit CMYK color space.
//!
////////////////////////////////////////////////////////////////////////////////
use super::{Hsl, Rgb};
use utilities::{lerp_u8, clamped};

use std::convert::From;
use std::fmt;
use std::u8;


////////////////////////////////////////////////////////////////////////////////
// Cmyk
////////////////////////////////////////////////////////////////////////////////
/// The encoded CMYK color.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone, Copy, Default)]
pub struct Cmyk {
	/// The cyan component.
	pub c: u8,
	/// The magenta component.
	pub m: u8,
	/// The yellow component.
	pub y: u8,
	/// The key (black) component.
	pub k: u8,
}


impl Cmyk {
	/// Creates a new Cmyk color.
	pub fn new(
		cyan: u8, 
		magenta: u8, 
		yellow: u8,
		key: u8) 
		-> Self 
	{
		Cmyk {c: cyan, m: magenta, y: yellow, k: key}
	}

	/// Returns the cyan component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Cmyk;
	/// 
	/// let c = Cmyk {c: 10, m: 20, y: 30, k: 40};
	///
	/// assert_eq!(c.cyan(), 10);
	/// ```
	pub fn cyan(&self) -> u8 {
		self.c
	}
	
	/// Returns the magenta component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Cmyk;
	/// 
	/// let c = Cmyk {c: 10, m: 20, y: 30, k: 40};
	///
	/// assert_eq!(c.magenta(), 20);
	/// ```
	pub fn magenta(&self) -> u8 {
		self.m
	}
	
	/// Returns the yellow component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Cmyk;
	/// 
	/// let c = Cmyk {c: 10, m: 20, y: 30, k: 40};
	///
	/// assert_eq!(c.yellow(), 30);
	/// ```
	pub fn yellow(&self) -> u8 {
		self.y
	}

	/// Returns the key component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Cmyk;
	/// 
	/// let c = Cmyk {c: 10, m: 20, y: 30, k: 40};
	///
	/// assert_eq!(c.key(), 40);
	/// ```
	pub fn key(&self) -> u8 {
		self.k
	}
	
	/// Sets the cyan component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Cmyk;
	/// 
	/// let mut c = Cmyk {c: 10, m: 20, y: 30, k: 40};
	/// c.set_cyan(99);
	///
	/// assert_eq!(c.cyan(), 99);
	/// ```
	pub fn set_cyan(&mut self, value: u8) {
		self.c = value;
	}
	
	/// Sets the magenta component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Cmyk;
	/// 
	/// let mut c = Cmyk {c: 10, m: 20, y: 30, k: 40};
	/// c.set_magenta(99);
	///
	/// assert_eq!(c.magenta(), 99);
	/// ```
	pub fn set_magenta(&mut self, value: u8) {
		self.m = value;
	}


	/// Sets the yellow component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Cmyk;
	/// 
	/// let mut c = Cmyk {c: 10, m: 20, y: 30, k: 40};
	/// c.set_yellow(99);
	///
	/// assert_eq!(c.yellow(), 99);
	/// ```
	pub fn set_yellow(&mut self, value: u8) {
		self.y = value;
	}

	/// Sets the key component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Cmyk;
	/// 
	/// let mut c = Cmyk {c: 10, m: 20, y: 30, k: 40};
	/// c.set_key(99);
	///
	/// assert_eq!(c.key(), 99);
	/// ```
	pub fn set_key(&mut self, value: u8) {
		self.k = value;
	}

	/// Returns an array containing the [C, M, Y, K] components octets.
	pub fn octets(&self) -> [u8; 4] {
		[self.c, self.m, self.y, self.k]
	}

	/// Returns an array containing the [C, M, Y, K] component ratios.
	pub fn ratios(&self) -> [f32; 4] {
		let max = u8::MAX as f32;
		[
			self.c as f32 / max,
			self.m as f32 / max, 
			self.y as f32 / max,
			self.k as f32 / max,
		]
	}

	/// Returns the CMYK hex code.
	pub fn hex(&self) -> u32 {
		(self.c as u32) << 24 | 
		(self.m as u32) << 16 | 
		(self.y as u32) << 8 | 
		(self.k as u32)
	}

	/// Performs a CMYK component-wise linear interpolation between the colors 
	/// `start` and `end`, returning the color located at the ratio given by 
	/// `amount`, which is clamped between 1 and 0.
	///
	/// # Examples
	///
	/// ```rust
	/// # use rampeditor::color::Cmyk;
	/// let c1 = Cmyk {c: 0, m: 10, y: 20, k: 15};
	/// let c2 = Cmyk {c: 100, m: 0, y: 80, k: 115};
	///
	/// let c = Cmyk::lerp(c1, c2, 0.5);
	/// assert_eq!(c, Cmyk {c: 50, m: 5, y: 50, k: 65});
	/// ```
	///
	/// ```rust
	/// # use rampeditor::color::Cmyk;
	/// let c1 = Cmyk {c: 189, m: 44, y: 23, k: 190};
	/// let c2 = Cmyk {c: 35, m: 255, y: 180, k: 74};
	///
	/// let a = Cmyk::lerp(c1, c2, 0.42);
	/// let y = Cmyk::lerp(c2, c1, 0.58);
	/// assert_eq!(a, y); // Reversed argument order inverts the ratio.
	/// ```
	pub fn lerp<C>(start: C, end: C, amount: f32) -> Self 
		where C: Into<Self> + Sized
	{
		if !amount.is_finite() {
			panic!("invalid argument at Cmyk::lerp(_, _, {:?}", amount);
		}
		let s = start.into();
		let e = end.into();
		Cmyk {
			c: lerp_u8(s.c, e.c, amount),
			m: lerp_u8(s.m, e.m, amount),
			y: lerp_u8(s.y, e.y, amount),
			k: lerp_u8(s.k, e.k, amount),
		}
	}

	/// Returns the distance between the given colors in CMYK color space.
	pub fn distance<C>(start: C, end: C) -> f32 
		where C: Into<Self> + Sized
	{
		let s = start.into();
		let e = end.into();
		
		let c = (s.c - e.c) as f32;
		let m = (s.m - e.m) as f32;
		let y = (s.y - e.y) as f32;
		let k = (s.k - e.k) as f32;

		(c*c + m*m + y*y + k*k).sqrt()
	}
}


impl fmt::Display for Cmyk {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:?}", self)
	}
}


impl fmt::UpperHex for Cmyk {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "#{:02X}{:02X}{:02X}{:02X}", self.c, self.m, self.y, self.k)
	}
}


impl fmt::LowerHex for Cmyk {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "#{:02x}{:02x}{:02x}{:02x}", self.c, self.m, self.y, self.k)
	}
}



////////////////////////////////////////////////////////////////////////////////
// Cmyk conversions
////////////////////////////////////////////////////////////////////////////////
impl From<u32> for Cmyk {
	fn from(hex: u32) -> Self {
		Cmyk {
			c: ((hex & 0xFF000000) >> 24) as u8,
			m: ((hex & 0x00FF0000) >> 16) as u8,
			y: ((hex & 0x0000FF00) >> 8) as u8,
			k: ((hex & 0x000000FF)) as u8,
		}
	}
}


impl From<[u8; 4]> for Cmyk {
	fn from(octets: [u8; 4]) -> Self {
		Cmyk {
			c: octets[0],
			m: octets[1],
			y: octets[2],
			k: octets[3],
		}
	}
}


impl From<[f32; 4]> for Cmyk {
	fn from(ratios: [f32; 4]) -> Self {
		Cmyk {
			c: (u8::MAX as f32 * clamped(ratios[0], 0f32, 1f32)) as u8,
			m: (u8::MAX as f32 * clamped(ratios[1], 0f32, 1f32)) as u8,
			y: (u8::MAX as f32 * clamped(ratios[2], 0f32, 1f32)) as u8,
			k: (u8::MAX as f32 * clamped(ratios[3], 0f32, 1f32)) as u8,
		}
	}
}


impl From<Rgb> for Cmyk {
	fn from(rgb: Rgb) -> Self {
		let ratios = rgb.ratios();

		let mut max = ratios[0];
		if ratios[1] > max {max = ratios[1];}
		if ratios[2] > max {max = ratios[2];}

		let kn = 1f32 - max;
		let cn = (1f32 - ratios[0] - kn) / max;
		let mn = (1f32 - ratios[1] - kn) / max;
		let yn = (1f32 - ratios[2] - kn) / max;

		Cmyk {
			c: (cn * u8::MAX as f32) as u8,
			m: (mn * u8::MAX as f32) as u8,
			y: (yn * u8::MAX as f32) as u8,
			k: (kn * u8::MAX as f32) as u8,
		}

	}
}


impl From<Hsl> for Cmyk {
	fn from(hsl: Hsl) -> Self {
		Cmyk::from(Rgb::from(hsl))
	}
}
