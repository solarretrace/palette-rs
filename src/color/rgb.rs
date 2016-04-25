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
//! Defines a 24-bit RGB color space.
//!
////////////////////////////////////////////////////////////////////////////////
use super::{Cmyk, Hsl, Hsv, Xyz};
use utilities::{lerp_u8, clamped};

use std::convert::From;
use std::fmt;
use std::u8;

////////////////////////////////////////////////////////////////////////////////
// Rgb
////////////////////////////////////////////////////////////////////////////////
/// The encoded RGB color.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone, Copy, Default)]
pub struct Rgb {
	/// The red component.
	pub r: u8,
	/// The green component.
	pub g: u8,
	/// The blue component.
	pub b: u8,
}


impl Rgb {
	/// Creates a new Rgb color.
	pub fn new(red: u8, green: u8, blue: u8) -> Self {
		Rgb {r: red, g: green, b: blue}
	}

	/// Returns the red component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Rgb;
	/// 
	/// let c = Rgb {r: 10, g: 20, b: 30};
	///
	/// assert_eq!(c.red(), 10);
	/// ```
	pub fn red(&self) -> u8 {
		self.r
	}
	
	/// Returns the green component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Rgb;
	/// 
	/// let c = Rgb {r: 10, g: 20, b: 30};
	///
	/// assert_eq!(c.green(), 20);
	/// ```
	pub fn green(&self) -> u8 {
		self.g
	}
	
	/// Returns the blue component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Rgb;
	/// 
	/// let c = Rgb {r: 10, g: 20, b: 30};
	///
	/// assert_eq!(c.blue(), 30);
	/// ```
	pub fn blue(&self) -> u8 {
		self.b
	}
	
	/// Sets the red component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Rgb;
	/// 
	/// let mut c = Rgb {r: 10, g: 20, b: 30};
	/// c.set_red(99);
	///
	/// assert_eq!(c.red(), 99);
	/// ```
	pub fn set_red(&mut self, value: u8) {
		self.r = value;
	}
	
	/// Sets the green component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Rgb;
	/// 
	/// let mut c = Rgb {r: 10, g: 20, b: 30};
	/// c.set_green(99);
	///
	/// assert_eq!(c.green(), 99);
	/// ```
	pub fn set_green(&mut self, value: u8) {
		self.g = value;
	}


	/// Sets the blue component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Rgb;
	/// 
	/// let mut c = Rgb {r: 10, g: 20, b: 30};
	/// c.set_blue(99);
	///
	/// assert_eq!(c.blue(), 99);
	/// ```
	pub fn set_blue(&mut self, value: u8) {
		self.b = value;
	}

	/// Returns an array containing the [R, G, B] component octets.
	pub fn octets(&self) -> [u8; 3] {
		[self.r, self.g, self.b]
	}

	/// Returns an array containing the [R, G, B] component ratios.
	pub fn ratios(&self) -> [f32; 3] {
		let max = u8::MAX as f32;
		[
			self.r as f32 / max,
			self.g as f32 / max, 
			self.b as f32 / max,
		]
	}

	/// Returns the RGB hex code.
	pub fn hex(&self) -> u32 {
		(self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
	}

	/// Performs an RGB component-wise linear interpolation between the colors 
	/// `start` and `end`, returning the color located at the ratio given by 
	/// `amount`, which is clamped between 1 and 0.
	///
	/// # Examples
	///
	/// ```rust
	/// # use rampeditor::color::Rgb;
	/// let c1 = Rgb {r: 0, g: 10, b: 20};
	/// let c2 = Rgb {r: 100, g: 0, b: 80};
	///
	/// let c = Rgb::lerp(c1, c2, 0.5);
	/// assert_eq!(c, Rgb {r: 50, g: 5, b: 50});
	/// ```
	///
	/// ```rust
	/// # use rampeditor::color::Rgb;
	/// let c1 = Rgb {r: 189, g: 44, b: 23};
	/// let c2 = Rgb {r: 35, g: 255, b: 180};
	///
	/// let a = Rgb::lerp(c1, c2, 0.42);
	/// let b = Rgb::lerp(c2, c1, 0.58);
	/// assert_eq!(a, b); // Reversed argument order inverts the ratio.
	/// ```
	pub fn lerp<C>(start: C, end: C, amount: f32) -> Self 
		where C: Into<Self> + Sized
	{
		if !amount.is_finite() {
			panic!("invalid argument at Rgb::lerp(_, _, {:?}", amount);
		}
		let s = start.into();
		let e = end.into();
		Rgb {
			r: lerp_u8(s.r, e.r, amount),
			g: lerp_u8(s.g, e.g, amount),
			b: lerp_u8(s.b, e.b, amount),
		}
	}

	/// Returns the distance between the given colors in RGB color space.
	pub fn distance<C>(start: C, end: C) -> f32 
		where C: Into<Self> + Sized
	{
		let s = start.into();
		let e = end.into();
		
		let r = (s.r - e.r) as f32;
		let g = (s.g - e.g) as f32;
		let b = (s.b - e.b) as f32;

		(r*r + g*g + b*b).sqrt()
	}
}



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


////////////////////////////////////////////////////////////////////////////////
// Rgb conversions
////////////////////////////////////////////////////////////////////////////////
impl From<u32> for Rgb {
	fn from(hex: u32) -> Self {
		Rgb {
			r: ((hex & 0xFF0000) >> 16) as u8,
			g: ((hex & 0x00FF00) >> 8) as u8,
			b: ((hex & 0x0000FF)) as u8,
		}
	}
}


impl From<[u8; 3]> for Rgb {
	fn from(octets: [u8; 3]) -> Self {
		Rgb {
			r: octets[0],
			g: octets[1],
			b: octets[2],
		}
	}
}

impl From<[f32; 3]> for Rgb {
	fn from(ratios: [f32; 3]) -> Self {
		Rgb {
			r: (u8::MAX as f32 * clamped(ratios[0], 0.0, 1.0)) as u8,
			g: (u8::MAX as f32 * clamped(ratios[1], 0.0, 1.0)) as u8,
			b: (u8::MAX as f32 * clamped(ratios[2], 0.0, 1.0)) as u8,
		}
	}
}


impl From<Cmyk> for Rgb {
	fn from(cmyk: Cmyk) -> Self {
		let ratios = cmyk.ratios();
		let cn = 1.0 - ratios[0];
		let mn = 1.0 - ratios[1];
		let yn = 1.0 - ratios[2];
		let kn = 1.0 - ratios[3];

		Rgb {
			r: (u8::MAX as f32 * cn * kn + 0.5) as u8,
			g: (u8::MAX as f32 * mn * kn + 0.5) as u8,
			b: (u8::MAX as f32 * yn * kn + 0.5) as u8,
		}
	}
}


impl From<Hsl> for Rgb {
	fn from(hsl: Hsl) -> Self {
		let (h, s, l) = (hsl.hue(), hsl.saturation(), hsl.lightness());

		// Compute intermediate values.
		let ci: f32 = s * (1.0 - (2.0 * l - 1.0).abs());
		let xi: f32 = ci * (1.0 - (h / 60.0 % 2.0 - 1.0).abs());
		let mi: f32 = l - ci / 2.0;

		// Scale and cast.
		let c = ((u8::MAX as f32) * ci) as u8;
		let x = ((u8::MAX as f32) * xi) as u8;
		let m = ((u8::MAX as f32) * mi) as u8;

		// Use hue hextant to select RGB color.
		match h {
			h if   0.0 <= h && h <  60.0 => Rgb::new(c+m, x+m,   m),
			h if  60.0 <= h && h < 120.0 => Rgb::new(x+m, c+m,   m),
			h if 120.0 <= h && h < 180.0 => Rgb::new(  m, c+m, x+m),
			h if 180.0 <= h && h < 240.0 => Rgb::new(  m, x+m, c+m),
			h if 240.0 <= h && h < 300.0 => Rgb::new(x+m,   m, c+m),
			h if 300.0 <= h && h < 360.0 => Rgb::new(c+m,   m, x+m),
			_ => unreachable!()
		}		
	}
}

impl From<Hsv> for Rgb {
	fn from(hsv: Hsv) -> Self {
		let (h, s, v) = (hsv.hue(), hsv.saturation(), hsv.value());

		let c = v * s;
		let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
		let m = v - c;

		let (ri, gi, bi) = match h {
			h if   0.0 <= h && h <  60.0 => (  c,   x, 0.0),
			h if  60.0 <= h && h < 120.0 => (  x,   c, 0.0),
			h if 120.0 <= h && h < 180.0 => (0.0,   c,   x),
			h if 180.0 <= h && h < 240.0 => (0.0,   x,   c),
			h if 240.0 <= h && h < 300.0 => (  x, 0.0,   c),
			h if 300.0 <= h && h < 360.0 => (  c, 0.0,   x),
			_ => unreachable!()
		};

		Rgb {
			r: ((ri + m) * (u8::MAX as f32)) as u8,
			g: ((gi + m) * (u8::MAX as f32)) as u8,
			b: ((bi + m) * (u8::MAX as f32)) as u8,
		}
	}
}

impl From<Xyz> for Rgb {
	fn from(xyz: Xyz) -> Self {
		let (x, y, z) = (xyz.x(), xyz.y(), xyz.z()); 

		let ri = x *  3.2404542 + y * -1.5371385 + z * -0.4985314;
		let gi = x * -0.9692660 + y *  1.8760108 + z *  0.0415560;
		let bi = x *  0.0556434 + y * -0.2040259 + z *  1.0572252;

		Rgb {
			r: (ri * u8::MAX as f32) as u8,
			g: (gi * u8::MAX as f32) as u8,
			b: (bi * u8::MAX as f32) as u8,
		}
	}
}
