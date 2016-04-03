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
use utilities::{lerp_f32, clamped};

use std::convert::From;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////
// HslChannel
////////////////////////////////////////////////////////////////////////////////
/// The type of a single RGB channel.
pub type HslChannel = f32;

////////////////////////////////////////////////////////////////////////////////
// Hsl
////////////////////////////////////////////////////////////////////////////////
/// The encoded RGB color.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy, Default)]
pub struct Hsl {
	/// The hue channel.
	h: HslChannel,
	/// The saturation channel.
	s: HslChannel,
	/// The lightness channel.
	l: HslChannel,
}


impl Hsl {
	/// Creates a new Hsl color.
	pub fn new(hue: HslChannel, saturation: HslChannel, lightness: HslChannel,) -> Self {
		let mut hsl = Hsl {h: 0.0, s: 0.0, l: 0.0};
		hsl.set_hue(hue);
		hsl.set_saturation(saturation);
		hsl.set_lightness(lightness);
		hsl
	}

	/// Returns the hue.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Hsl;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let c = Hsl::new(10.0, 0.2, 0.3);
	///
	/// assert!(nearly_equal(c.hue(), 10.0));
	/// ```
	pub fn hue(&self) -> HslChannel {
		self.h
	}
	
	/// Returns the saturation.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Hsl;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let c = Hsl::new(10.0, 0.2, 0.3);
	///
	/// assert!(nearly_equal(c.saturation(), 0.2));
	/// ```
	pub fn saturation(&self) -> HslChannel {
		self.s
	}
	
	/// Returns the lightness.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Hsl;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let c = Hsl::new(10.0, 0.2, 0.3);
	///
	/// assert!(nearly_equal(c.lightness(), 0.3));
	/// ```
	pub fn lightness(&self) -> HslChannel {
		self.l
	}
	
	/// Sets the hue.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Hsl;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let mut c = Hsl::new(10.0, 0.2, 0.3);
	/// c.set_hue(99.0);
	///
	/// assert!(nearly_equal(c.hue(), 99.0));
	/// ```
	pub fn set_hue(&mut self, value: HslChannel) {
		self.h = (value + (if value < 0.0 {360.0} else {0.0})) % 360.0;
	}
	
	/// Sets the saturation.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Hsl;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let mut c = Hsl::new(10.0, 0.2, 0.3);
	/// c.set_saturation(0.99);
	///
	/// assert!(nearly_equal(c.saturation(), 0.99));
	/// ```
	pub fn set_saturation(&mut self, value: HslChannel) {
		self.s = clamped(value, 0.0, 1.0);;
	}


	/// Sets the lightness.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Hsl;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let mut c = Hsl::new(10.0, 0.2, 0.3);
	/// c.set_lightness(0.99);
	///
	/// assert!(nearly_equal(c.lightness(), 0.99));
	/// ```
	pub fn set_lightness(&mut self, value: HslChannel) {
		self.l = clamped(value, 0.0, 1.0);
	}

	/// Returns an array containing the [H, S, L] component channels.
	pub fn components(&self) -> [HslChannel; 3] {
		[self.h, self.s, self.l]
	}

	/// Performs an HSL component-wise linear interpolation between the colors 
	/// `start` and `end`, returning the color located at the ratio given by 
	/// `amount`, which is clamped between 1 and 0.
	///
	/// # Examples
	///
	/// ```rust
	/// # use rampeditor::color::Hsl;
	/// # use rampeditor::utilities::nearly_equal;
	///
	/// let c1 = Hsl::new(45.0, 0.5, 0.8);
	/// let c2 = Hsl::new(110.0, 0.4, 0.9);
	///
	/// let c = Hsl::lerp(c1, c2, 0.5);
	/// assert!(nearly_equal(c.hue(), 77.5));
	/// assert!(nearly_equal(c.saturation(), 0.45));
	/// assert!(nearly_equal(c.lightness(), 0.85));
	/// ```
	///
	/// ```rust
	/// # use rampeditor::color::Hsl;
	/// # use rampeditor::utilities::nearly_equal;
	/// let c1 = Hsl::new(182.0, 0.44, 0.43);
	/// let c2 = Hsl::new(35.0, 0.24, 0.80);
	///
	/// let a = Hsl::lerp(c1, c2, 0.42);
	/// let b = Hsl::lerp(c2, c1, 0.58);
	/// // Reversed argument order inverts the ratio.
	/// assert!(nearly_equal(a.hue(), b.hue()));
	/// assert!(nearly_equal(a.saturation(), b.saturation()));
	/// assert!(nearly_equal(a.lightness(), b.lightness()));
	/// ```
	pub fn lerp<C>(start: C, end: C, amount: f32) -> Self 
		where C: Into<Self> + Sized
	{
		let s = start.into();
		let e = end.into();
		Hsl {
			h: lerp_f32(s.h, e.h, amount),
			s: lerp_f32(s.s, e.s, amount),
			l: lerp_f32(s.l, e.l, amount),
		}
	}

	/// Returns the distance between the given colors in RGB color space.
	pub fn distance<C>(start: C, end: C) -> f32 
		where C: Into<Self> + Sized
	{
		let s = start.into();
		let e = end.into();
		
		let csx = s.l * s.h.cos() * 2.0;
		let csy = s.l * s.h.sin() * 2.0;
		let cex = e.l * e.h.cos() * 2.0;
		let cey = e.l * e.h.sin() * 2.0;

		let s = s.s - e.s;
		let x = csx - cex;
		let y = csy - cey;

		(s*s + x*x + y*y).sqrt() / 6f32.sqrt()
	}
}


impl From<[HslChannel; 3]> for Hsl {
	fn from(components: [HslChannel; 3]) -> Hsl {
		Hsl {
			h: components[0],
			s: components[1],
			l: components[2],
		}
	}
}

impl From<(HslChannel, HslChannel, HslChannel)> for Hsl {
	fn from(components: (HslChannel, HslChannel, HslChannel)) -> Hsl {
		Hsl {
			h: components.0,
			s: components.1,
			l: components.2,
		}
	}
}

impl fmt::Display for Hsl {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:?}", self)
	}
}