// The MIT License (MIT)
// 
// Copyright (c) 2016 Skylor R. Schermer
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditiony:
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
//! Defines a 96-bit XYZ color space.
//!
////////////////////////////////////////////////////////////////////////////////
use super::{Cmyk, Hsl, Hsv, Rgb};
use utilities::{lerp_f32, clamped};

use std::convert::From;
use std::fmt;


////////////////////////////////////////////////////////////////////////////////
// Xyz
////////////////////////////////////////////////////////////////////////////////
/// The encoded XYZ color.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy, Default)]
pub struct Xyz {
	/// The x component.
	x: f32,
	/// The y component.
	y: f32,
	/// The z component.
	z: f32,
}


impl Xyz {
	/// Creates a new Xyz color.
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		if !x.is_finite() 
			|| !y.is_finite() 
			|| !z.is_finite()
		{
			panic!("invalid argument at Xyz::new({:?}, {:?}, {:?})",
				x, y, z
			);
		}

		let mut xyz = Xyz {x: 0.0, y: 0.0, z: 0.0};
		xyz.set_x(x);
		xyz.set_y(y);
		xyz.set_z(z);
		xyz
	}

	/// Returns the x component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Xyz;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let c = Xyz::new(1.0, 0.2, 0.3);
	///
	/// assert!(nearly_equal(c.x(), 1.0));
	/// ```
	pub fn x(&self) -> f32 {
		self.x
	}
	
	/// Returns the y component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Xyz;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let c = Xyz::new(10.0, 0.2, 0.3);
	///
	/// assert!(nearly_equal(c.y(), 0.2));
	/// ```
	pub fn y(&self) -> f32 {
		self.y
	}
	
	/// Returns the z component.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Xyz;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let c = Xyz::new(10.0, 0.2, 0.3);
	///
	/// assert!(nearly_equal(c.z(), 0.3));
	/// ```
	pub fn z(&self) -> f32 {
		self.z
	}
	
	/// Sets the x.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Xyz;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let mut c = Xyz::new(10.0, 0.2, 0.3);
	/// c.set_x(0.99);
	///
	/// assert!(nearly_equal(c.x(), 0.99));
	/// ```
	pub fn set_x(&mut self, x: f32) {
		if !x.is_finite() {
			panic!("invalid argument at Xyz::set_x({:?})", x);
		}
		self.x = clamped(x, 0.0, 1.0);
	}
	
	/// Sets the y.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Xyz;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let mut c = Xyz::new(10.0, 0.2, 0.3);
	/// c.set_y(0.99);
	///
	/// assert!(nearly_equal(c.y(), 0.99));
	/// ```
	pub fn set_y(&mut self, y: f32) {
		if !y.is_finite() {
			panic!("invalid argument at Xyz::set_y({:?})", y);
		}
		self.y = clamped(y, 0.0, 1.0);
	}


	/// Sets the z.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::color::Xyz;
	/// # use rampeditor::utilities::nearly_equal;
	/// 
	/// let mut c = Xyz::new(10.0, 0.2, 0.3);
	/// c.set_z(0.99);
	///
	/// assert!(nearly_equal(c.z(), 0.99));
	/// ```
	pub fn set_z(&mut self, z: f32) {
		if !z.is_finite() {
			panic!("invalid argument at Xyz::set_z({:?})", z);
		}
		self.z = clamped(z, 0.0, 1.0);
	}

	/// Returns an array containing the [X, Y, Z] components.
	pub fn components(&self) -> [f32; 3] {
		[self.x, self.y, self.z]
	}

	/// Performs an XYZ component-wise linear interpolation between the colors 
	/// `start` and `end`, returning the color located at the ratio given by 
	/// `amount`, which is clamped between 1 and 0.
	///
	/// # Examples
	///
	/// ```rust
	/// # use rampeditor::color::Xyz;
	/// # use rampeditor::utilities::nearly_equal;
	///
	/// let c1 = Xyz::new(0.45, 0.5, 0.8);
	/// let c2 = Xyz::new(0.11, 0.4, 0.9);
	///
	/// let c = Xyz::lerp(c1, c2, 0.5);
	/// assert!(nearly_equal(c.x(), 0.28));
	/// assert!(nearly_equal(c.y(), 0.45));
	/// assert!(nearly_equal(c.z(), 0.85));
	/// ```
	///
	/// ```rust
	/// # use rampeditor::color::Xyz;
	/// # use rampeditor::utilities::nearly_equal;
	/// let c1 = Xyz::new(0.182, 0.44, 0.43);
	/// let c2 = Xyz::new(0.35, 0.24, 0.80);
	///
	/// let a = Xyz::lerp(c1, c2, 0.42);
	/// let b = Xyz::lerp(c2, c1, 0.58);
	/// // Reversed argument order inverts the ratio.
	/// assert!(nearly_equal(a.x(), b.x()));
	/// assert!(nearly_equal(a.y(), b.y()));
	/// assert!(nearly_equal(a.z(), b.z()));
	/// ```
	pub fn lerp<C>(start: C, end: C, amount: f32) -> Self 
		where C: Into<Self> + Sized
	{
		if !amount.is_finite() {
			panic!("invalid argument at Xyz::lerp(_, _, {:?}", amount);
		}
		let s = start.into();
		let e = end.into();
		Xyz {
			x: lerp_f32(s.x, e.x, amount),
			y: lerp_f32(s.y, e.y, amount),
			z: lerp_f32(s.z, e.z, amount),
		}
	}

	/// Returns the distance between the given colors in XYZ color space.
	pub fn distance<C>(start: C, end: C) -> f32 
		where C: Into<Self> + Sized
	{
		let s = start.into();
		let e = end.into();
		
		let x = (s.x - e.x) as f32;
		let y = (s.y - e.y) as f32;
		let z = (s.z - e.z) as f32;

		(x*x + y*y + z*z).sqrt()
	}
}


impl fmt::Display for Xyz {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:?}", self)
	}
}


////////////////////////////////////////////////////////////////////////////////
// Xyzconversions
////////////////////////////////////////////////////////////////////////////////
impl From<[f32; 3]> for Xyz {
	fn from(components: [f32; 3]) -> Self {
		Xyz {
			x: components[0],
			y: components[1],
			z: components[2],
		}
	}
}


impl From<Cmyk> for Xyz {
	fn from(cmyk: Cmyk) -> Self {
		Xyz::from(Rgb::from(cmyk))
	}
}

impl From<Hsl> for Xyz {
	fn from(hsl: Hsl) -> Self {
		Xyz::from(Rgb::from(hsl))
	}
}

impl From<Hsv> for Xyz {
	fn from(hsv: Hsv) -> Self {
		Xyz::from(Rgb::from(hsv))
	}
}

impl From<Rgb> for Xyz {
	fn from(rgb: Rgb) -> Self {
		let m = rgb.ratios(); 

		Xyz {
			x: m[0] * 0.4124564 + m[1] * 0.3575761 + m[2] * 0.1804375,
			y: m[0] * 0.2126729 + m[1] * 0.7151522 + m[2] * 0.0721750,
			z: m[0] * 0.0193339 + m[1] * 0.1191920 + m[2] * 0.9503041,
		}
	}
}

