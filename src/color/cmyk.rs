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
use utilities::lerp_u8;

use std::convert::From;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////
// CmykChannel
////////////////////////////////////////////////////////////////////////////////
/// The type of a single CMYK channel.
pub type CmykChannel = u8;



////////////////////////////////////////////////////////////////////////////////
// Cmyk
////////////////////////////////////////////////////////////////////////////////
/// The encoded CMYK color.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone, Copy, Default)]
pub struct Cmyk {
	/// The cyan channel.
	pub c: CmykChannel,
	/// The magenta channel.
	pub m: CmykChannel,
	/// The yellow channel.
	pub y: CmykChannel,
	/// The key (black) channel.
	pub k: CmykChannel,
}


impl Cmyk {
	/// Creates a new Cmyk color.
	pub fn new(
		cyan: CmykChannel, 
		magenta: CmykChannel, 
		yellow: CmykChannel,
		key: CmykChannel) 
		-> Self 
	{
		Cmyk {c: cyan, m: magenta, y: yellow, k: key}
	}

	/// Returns the cyan channel.
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
	pub fn cyan(&self) -> CmykChannel {
		self.c
	}
	
	/// Returns the magenta channel.
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
	pub fn magenta(&self) -> CmykChannel {
		self.m
	}
	
	/// Returns the yellow channel.
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
	pub fn yellow(&self) -> CmykChannel {
		self.y
	}

	/// Returns the key channel.
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
	pub fn key(&self) -> CmykChannel {
		self.k
	}
	
	/// Sets the cyan channel.
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
	pub fn set_cyan(&mut self, value: CmykChannel) {
		self.c = value;
	}
	
	/// Sets the magenta channel.
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
	pub fn set_magenta(&mut self, value: CmykChannel) {
		self.m = value;
	}


	/// Sets the yellow channel.
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
	pub fn set_yellow(&mut self, value: CmykChannel) {
		self.y = value;
	}

	/// Sets the key channel.
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
	pub fn set_key(&mut self, value: CmykChannel) {
		self.k = value;
	}

	/// Returns an array containing the [R, G, B] component channels.
	pub fn components(&self) -> [CmykChannel; 4] {
		[self.c, self.m, self.y, self.k]
	}

	/// Performs an CMYK component-wise linear interpolation between the colors 
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



impl From<u32> for Cmyk {
	fn from(hex: u32) -> Cmyk {
		Cmyk {
			c: ((hex & 0xFF000000) >> 24) as CmykChannel,
			m: ((hex & 0x00FF0000) >> 16) as CmykChannel,
			y: ((hex & 0x0000FF00) >> 8) as CmykChannel,
			k: ((hex & 0x000000FF)) as CmykChannel,
		}
	}
}

impl From<[CmykChannel; 4]> for Cmyk {
	fn from(components: [CmykChannel; 4]) -> Cmyk {
		Cmyk {
			c: components[0],
			m: components[1],
			y: components[2],
			k: components[3],
		}
	}
}

