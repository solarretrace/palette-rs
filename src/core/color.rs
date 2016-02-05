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

//! Defines the color space that the Palette is compatable with.
use super::utilities::lerp;

use std::fmt;

////////////////////////////////////////////////////////////////////////////////
// Color
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single RGB color.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:?}", self)
	}
}

impl fmt::UpperHex for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
	}
}

impl fmt::LowerHex for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
	}
}

/// Performs an rgb component-wise linear interpolation between the colors 
/// `start` and `end`, returning the color located at the ratio given by 
/// `amount`, which is clamped between 1 and 0.
///
/// # Examples
///
/// ```rust
/// # use rampeditor::core::color::{Color, lerp_rgb};
/// let c1 = Color(0, 10, 20);
/// let c2 = Color(100, 0, 80);
///
/// let c = lerp_rgb(c1, c2, 0.5);
/// assert_eq!(c, Color(50, 5, 50));
/// ```
///
/// ```rust
/// # use rampeditor::core::color::{Color, lerp_rgb};
/// let c1 = Color(189, 44, 23);
/// let c2 = Color(35, 255, 180);
///
/// let a = lerp_rgb(c1, c2, 0.42);
/// let b = lerp_rgb(c2, c1, 0.58);
/// assert_eq!(a, b); // Reversed argument order inverts the ratio.
/// ```
pub fn lerp_rgb(start: Color, end: Color, amount: f32) -> Color {
	Color(
		lerp(start.0, end.0, amount),
		lerp(start.1, end.1, amount),
		lerp(start.2, end.2, amount)
	)
}