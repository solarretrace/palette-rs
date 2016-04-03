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
use utilities::lerp;

use std::convert::From;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////
// RgbChannel
////////////////////////////////////////////////////////////////////////////////
/// The type of a single RGB channel.
pub type RgbChannel = u8;


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
	/// use rampeditor::color::Rgb;
	/// 
	/// let c = Rgb::new(10, 20, 30);
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
	/// use rampeditor::color::Rgb;
	/// 
	/// let c = Rgb::new(10, 20, 30);
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
	/// use rampeditor::color::Rgb;
	/// 
	/// let c = Rgb::new(10, 20, 30);
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
	/// use rampeditor::color::Rgb;
	/// 
	/// let mut c = Rgb::new(10, 20, 30);
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
	/// use rampeditor::color::Rgb;
	/// 
	/// let mut c = Rgb::new(10, 20, 30);
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
	/// use rampeditor::color::Rgb;
	/// 
	/// let mut c = Rgb::new(10, 20, 30);
	/// c.set_blue(99);
	///
	/// assert_eq!(c.blue(), 99);
	/// ```
	pub fn set_blue(&mut self, value: RgbChannel) {
		self.b = value;
	}

	/// Performs an RGB component-wise linear interpolation between the colors 
	/// `start` and `end`, returning the color located at the ratio given by 
	/// `amount`, which is clamped between 1 and 0.
	///
	/// # Examples
	///
	/// ```rust
	/// # use rampeditor::color::Rgb;
	/// let c1 = Rgb::new(0, 10, 20);
	/// let c2 = Rgb::new(100, 0, 80);
	///
	/// let c = Rgb::lerp(c1, c2, 0.5);
	/// assert_eq!(c, Rgb::new(50, 5, 50));
	/// ```
	///
	/// ```rust
	/// # use rampeditor::color::Rgb;;
	/// let c1 = Rgb::new(189, 44, 23);
	/// let c2 = Rgb::new(35, 255, 180);
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
			r: lerp(s.r, e.r, amount),
			g: lerp(s.g, e.g, amount),
			b: lerp(s.b, e.b, amount),
		}
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