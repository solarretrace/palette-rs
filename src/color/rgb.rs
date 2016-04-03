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
use super::Hsl;
use utilities::lerp_u8;

use std::convert::From;
use std::fmt;
use std::u8;

////////////////////////////////////////////////////////////////////////////////
// RgbChannel
////////////////////////////////////////////////////////////////////////////////
/// The type of a single RGB channel.
pub type RgbChannel = u8;

const RGB_CHANNEL_MAX: RgbChannel = u8::MAX;

////////////////////////////////////////////////////////////////////////////////
// Rgb
////////////////////////////////////////////////////////////////////////////////
/// The encoded RGB color.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone, Copy, Default)]
pub struct Rgb {
	/// The red channel.
	pub r: RgbChannel,
	/// The green channel.
	pub g: RgbChannel,
	/// The blue channel.
	pub b: RgbChannel,
}


impl Rgb {
	/// Creates a new Rgb color.
	pub fn new(red: RgbChannel, green: RgbChannel, blue: RgbChannel,) -> Self {
		Rgb {r: red, g: green, b: blue}
	}

	/// Returns the red channel.
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
	pub fn red(&self) -> RgbChannel {
		self.r
	}
	
	/// Returns the green channel.
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
	pub fn green(&self) -> RgbChannel {
		self.g
	}
	
	/// Returns the blue channel.
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
	pub fn blue(&self) -> RgbChannel {
		self.b
	}
	
	/// Sets the red channel.
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
	pub fn set_red(&mut self, value: RgbChannel) {
		self.r = value;
	}
	
	/// Sets the green channel.
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
	pub fn set_green(&mut self, value: RgbChannel) {
		self.g = value;
	}


	/// Sets the blue channel.
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
	pub fn set_blue(&mut self, value: RgbChannel) {
		self.b = value;
	}

	/// Returns an array containing the [R, G, B] component channels.
	pub fn components(&self) -> [RgbChannel; 3] {
		[self.r, self.g, self.b]
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



impl From<u32> for Rgb {
	fn from(hex: u32) -> Rgb {
		Rgb {
			r: ((hex & 0xFF0000) >> 16) as RgbChannel,
			g: ((hex & 0x00FF00) >> 8) as RgbChannel,
			b: ((hex & 0x0000FF)) as RgbChannel,
		}
	}
}

impl From<[RgbChannel; 3]> for Rgb {
	fn from(components: [RgbChannel; 3]) -> Rgb {
		Rgb {
			r: components[0],
			g: components[1],
			b: components[2],
		}
	}
}

impl From<Hsl> for Rgb {
	fn from(hsl: Hsl) -> Rgb {
		let (h, s, l) = (hsl.hue(), hsl.saturation(), hsl.lightness());

		let ci: f32 = s * (1.0 - (2.0 * l - 1.0).abs());
		let xi: f32 = ci * (1.0 - (h / 60.0 % 2.0 - 1.0).abs());
		let mi: f32 = l - ci / 2.0;

		let c = ((RGB_CHANNEL_MAX as f32) * ci) as RgbChannel;
		let x = ((RGB_CHANNEL_MAX as f32) * xi) as RgbChannel;
		let m = ((RGB_CHANNEL_MAX as f32) * mi) as RgbChannel;

		match h {
			h if   0.0 <= h && h <  60.0 => Rgb::new(c+m, x+m,   m),
			h if  60.0 <= h && h < 120.0 => Rgb::new(x+m, c+m,   m),
			h if 120.0 <= h && h < 180.0 => Rgb::new(  m, c+m, x+m),
			h if 180.0 <= h && h < 240.0 => Rgb::new(  m, x+m, c+m),
			h if 240.0 <= h && h < 300.0 => Rgb::new(x+m,   m, c+m),
			_ => Rgb::new(c+m, m, x+m),
		}
	}
}

