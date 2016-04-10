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
//! Defines a 96-bit HSV color space.
//!
////////////////////////////////////////////////////////////////////////////////
use super::{Cmyk, Hsl, Rgb, Xyz};
use utilities::{lerp_f32, clamped, nearly_equal};

use std::convert::From;
use std::fmt;


////////////////////////////////////////////////////////////////////////////////
// Hsv
////////////////////////////////////////////////////////////////////////////////
/// The encoded HSV color.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy, Default)]
pub struct Hsv {
	/// The hue component.
	h: f32,
	/// The saturation component.
	s: f32,
	/// The value component.
	v: f32,
}


impl Hsv {
	/// Creates a new Hsv color.
	pub fn new(hue: f32, saturation: f32, value: f32) -> Self {
		if !hue.is_finite() 
			|| !saturation.is_finite() 
			|| !value.is_finite()
		{
			panic!("invalid argument at Hsv::new({:?}, {:?}, {:?})",
				hue, saturation, value
			);
		}

		let mut hsv = Hsv {h: 0.0, s: 0.0, v: 0.0};
		hsv.set_hue(hue);
		hsv.set_saturation(saturation);
		hsv.set_value(value);
		hsv
	}

	/// Returns the hue.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Hsv;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let c = Hsv::new(10.0, 0.2, 0.3);
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
	/// # use rampeditor::color::Hsv;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let c = Hsv::new(10.0, 0.2, 0.3);
	///
	/// assert!(nearly_equal(c.saturation(), 0.2));
	/// ```
	pub fn saturation(&self) -> f32 {
		self.s
	}
	
	/// Returns the value.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Hsv;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let c = Hsv::new(10.0, 0.2, 0.3);
	///
	/// assert!(nearly_equal(c.value(), 0.3));
	/// ```
	pub fn value(&self) -> f32 {
		self.v
	}
	
	/// Sets the hue.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Hsv;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let mut c = Hsv::new(10.0, 0.2, 0.3);
	/// c.set_hue(99.0);
	///
	/// assert!(nearly_equal(c.hue(), 99.0));
	/// ```
	pub fn set_hue(&mut self, hue: f32) {
		if !hue.is_finite() {
			panic!("invalid argument at Hsv::set_hue({:?})", hue);
		}
		self.h = hue % 360.0;
	}
	
	/// Sets the saturation.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Hsv;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let mut c = Hsv::new(10.0, 0.2, 0.3);
	/// c.set_saturation(0.99);
	///
	/// assert!(nearly_equal(c.saturation(), 0.99));
	/// ```
	pub fn set_saturation(&mut self, saturation: f32) {
		if !saturation.is_finite() {
			panic!("invalid argument at Hsv::set_saturation({:?})", saturation);
		}
		self.s = clamped(saturation, 0.0, 1.0);
	}


	/// Sets the value.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Hsv;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let mut c = Hsv::new(10.0, 0.2, 0.3);
	/// c.set_value(0.99);
	///
	/// assert!(nearly_equal(c.value(), 0.99));
	/// ```
	pub fn set_value(&mut self, value: f32) {
		if !value.is_finite() {
			panic!("invalid argument at Hsv::set_value({:?})", value);
		}
		self.v = clamped(value, 0.0, 1.0);
	}

	/// Returns an array containing the [H, S, V] components.
	pub fn components(&self) -> [f32; 3] {
		[self.h, self.s, self.v]
	}

	/// Performs an HSV component-wise linear interpolation between the colors 
	/// `start` and `end`, returning the color located at the ratio given by 
	/// `amount`, which is clamped between 1 and 0.
	///
	/// # Examples
	///
	/// ```rust
	/// # use rampeditor::color::Hsv;
	/// # use rampeditor::utilities::nearly_equal;
	///
	/// let c1 = Hsv::new(45.0, 0.5, 0.8);
	/// let c2 = Hsv::new(110.0, 0.4, 0.9);
	///
	/// let c = Hsv::lerp(c1, c2, 0.5);
	/// assert!(nearly_equal(c.hue(), 77.5));
	/// assert!(nearly_equal(c.saturation(), 0.45));
	/// assert!(nearly_equal(c.value(), 0.85));
	/// ```
	///
	/// ```rust
	/// # use rampeditor::color::Hsv;
	/// # use rampeditor::utilities::nearly_equal;
	/// let c1 = Hsv::new(182.0, 0.44, 0.43);
	/// let c2 = Hsv::new(35.0, 0.24, 0.80);
	///
	/// let a = Hsv::lerp(c1, c2, 0.42);
	/// let b = Hsv::lerp(c2, c1, 0.58);
	/// // Reversed argument order inverts the ratio.
	/// assert!(nearly_equal(a.hue(), b.hue()));
	/// assert!(nearly_equal(a.saturation(), b.saturation()));
	/// assert!(nearly_equal(a.value(), b.value()));
	/// ```
	pub fn lerp<C>(start: C, end: C, amount: f32) -> Self 
		where C: Into<Self> + Sized
	{
		if !amount.is_finite() {
			panic!("invalid argument at Hsv::lerp(_, _, {:?}", amount);
		}
		let s = start.into();
		let e = end.into();
		Hsv {
			h: lerp_f32(s.h, e.h, amount),
			s: lerp_f32(s.s, e.s, amount),
			v: lerp_f32(s.v, e.v, amount),
		}
	}

	/// Returns the distance between the given colors in HSV color space.
	pub fn distance<C>(start: C, end: C) -> f32 
		where C: Into<Self> + Sized
	{
		let s = start.into();
		let e = end.into();
		
		let (shx, shy) = s.h.sin_cos();
		let (ehx, ehy) = e.h.sin_cos();
		let csx = s.v * shx * 2.0;
		let csy = s.v * shy * 2.0;
		let cex = e.v * ehx * 2.0;
		let cey = e.v * ehy * 2.0;

		let s = s.s - e.s;
		let x = csx - cex;
		let y = csy - cey;

		(s*s + x*x + y*y).sqrt() / 6.0f32.sqrt()
	}
}


impl fmt::Display for Hsv {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:?}", self)
	}
}


////////////////////////////////////////////////////////////////////////////////
// Hsv conversions
////////////////////////////////////////////////////////////////////////////////
impl From<[f32; 3]> for Hsv {
	fn from(components: [f32; 3]) -> Self {
		Hsv {
			h: components[0],
			s: components[1],
			v: components[2],
		}
	}
}

impl From<Cmyk> for Hsv {
	fn from(cmyk: Cmyk) -> Self {
		Hsv::from(Rgb::from(cmyk))
	}
}

impl From<Hsl> for Hsv {
	fn from(hsl: Hsl) -> Self {
		Hsv::from(Rgb::from(hsl))
	}
}

impl From<Rgb> for Hsv {
	fn from(rgb: Rgb) -> Self {
		// Find min, max, index of max, and delta.
		let ratios = rgb.ratios();
		let (min, max, max_index, _) = ratios
			.into_iter()
			.fold((ratios[0], ratios[0], 0, 0), |(min, max, i, c), &x| {
				match (x < min, x > max) {
					(true, false) => (x, max, i, c+1),
					(false, true) => (min, x, c, c+1),
					_ => (min, max, i, c+1)
				}
			});
		let delta = max - min;

		
		if nearly_equal(delta, 0.0) {
			// No need to compute saturation and hue for grayscale colors.
			Hsv {h: 0.0, s: 0.0, v: max}

		} else {

			// Compute saturation.
			let s = if nearly_equal(max, 0.0)  {
				0.0
			} else {
				delta / max
			};

			// Compute hue.
			let mut h = 60.0 * match max_index {
				0 => ((ratios[1] - ratios[2]) / delta) % 6.0,
				1 => (ratios[2] - ratios[0]) / delta + 2.0,
				2 => (ratios[0] - ratios[1]) / delta + 4.0,
				_ => unreachable!()
			};

			// Correct wrapping.
			h %= 360.0;
			if h < 0.0 {h += 360.0};
			
			Hsv {h: h, s: s, v: max}
		}

	}
}

impl From<Xyz> for Hsv {
	fn from(xyz: Xyz) -> Self {
		Hsv::from(Rgb::from(xyz))
	}
}
