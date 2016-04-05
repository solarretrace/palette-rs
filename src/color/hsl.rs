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
//! Defines a 96-bit HSL color space.
//!
////////////////////////////////////////////////////////////////////////////////
use super::{Cmyk, Rgb};
use utilities::{lerp_f32, clamped, nearly_equal};

use std::convert::From;
use std::fmt;


////////////////////////////////////////////////////////////////////////////////
// Hsl
////////////////////////////////////////////////////////////////////////////////
/// The encoded HSL color.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy, Default)]
pub struct Hsl {
	/// The hue component.
	h: f32,
	/// The saturation component.
	s: f32,
	/// The lightness component.
	l: f32,
}


impl Hsl {
	/// Creates a new Hsl color.
	pub fn new(hue: f32, saturation: f32, lightness: f32) -> Self {
		if !hue.is_finite() 
			|| !saturation.is_finite() 
			|| !lightness.is_finite()
		{
			panic!("invalid argument at Hsl::new({:?}, {:?}, {:?})",
				hue, saturation, lightness
			);
		}

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
	pub fn hue(&self) -> f32 {
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
	pub fn saturation(&self) -> f32 {
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
	pub fn lightness(&self) -> f32 {
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
	pub fn set_hue(&mut self, value: f32) {
		if !value.is_finite() {
			panic!("invalid argument at Hsl::set_hue({:?})", value);
		}
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
	pub fn set_saturation(&mut self, value: f32) {
		if !value.is_finite() {
			panic!("invalid argument at Hsl::set_saturation({:?})", value);
		}
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
	pub fn set_lightness(&mut self, value: f32) {
		if !value.is_finite() {
			panic!("invalid argument at Hsl::set_lightness({:?})", value);
		}
		self.l = clamped(value, 0.0, 1.0);
	}

	/// Returns an array containing the [H, S, L] components.
	pub fn components(&self) -> [f32; 3] {
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
		if !amount.is_finite() {
			panic!("invalid argument at Hsl::lerp(_, _, {:?}", amount);
		}
		let s = start.into();
		let e = end.into();
		Hsl {
			h: lerp_f32(s.h, e.h, amount),
			s: lerp_f32(s.s, e.s, amount),
			l: lerp_f32(s.l, e.l, amount),
		}
	}

	/// Returns the distance between the given colors in HSL color space.
	pub fn distance<C>(start: C, end: C) -> f32 
		where C: Into<Self> + Sized
	{
		let s = start.into();
		let e = end.into();
		
		let (shx, shy) = s.h.sin_cos();
		let (ehx, ehy) = e.h.sin_cos();
		let csx = s.l * shx * 2.0;
		let csy = s.l * shy * 2.0;
		let cex = e.l * ehx * 2.0;
		let cey = e.l * ehy * 2.0;

		let s = s.s - e.s;
		let x = csx - cex;
		let y = csy - cey;

		(s*s + x*x + y*y).sqrt() / 6f32.sqrt()
	}
}


impl fmt::Display for Hsl {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:?}", self)
	}
}


////////////////////////////////////////////////////////////////////////////////
// Hslconversions
////////////////////////////////////////////////////////////////////////////////
impl From<[f32; 3]> for Hsl {
	fn from(components: [f32; 3]) -> Self {
		Hsl {
			h: components[0],
			s: components[1],
			l: components[2],
		}
	}
}


impl From<(f32, f32, f32)> for Hsl {
	fn from(components: (f32, f32, f32)) -> Self {
		Hsl {
			h: components.0,
			s: components.1,
			l: components.2,
		}
	}
}


impl From<Cmyk> for Hsl {
	fn from(cmyk: Cmyk) -> Self {
		Hsl::from(Rgb::from(cmyk))
	}
}


impl From<Rgb> for Hsl {
	fn from(rgb: Rgb) -> Self {
		let ratios = rgb.ratios();

		let mut max = ratios[0];
		if ratios[1] > max {max = ratios[1];}
		if ratios[2] > max {max = ratios[2];}

		let mut min = ratios[0];
		if ratios[1] < min {min = ratios[1];}
		if ratios[2] < min {min = ratios[2];}

		let delta = max - min;

		let l = (max + min) / 2f32;

		let s = if nearly_equal(delta, 0f32) { 
			0f32
		} else {
			delta / (1f32 - (2f32 * l - 1f32))
		};

		let h = 60f32 * if nearly_equal(delta, 0f32) {
			0f32
		} else if max == ratios[0] {
			((ratios[1] + ratios[2]) / delta)
		} else if max == ratios[0] {
			(ratios[0] + ratios[2]) / delta + 2f32
		} else {
			(ratios[0] + ratios[1]) / delta + 4f32
		};

		Hsl {h: h, s: s, l: l}
	}
}

