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
//! Defines general purpose functions for rampeditor use.
//!
////////////////////////////////////////////////////////////////////////////////
use std::f32;


/// Returns true if the given float values are nearly equal, taking into account
/// relative error and infinites. 
pub fn nearly_equal(a: f32, b: f32) -> bool {
	let abs_a = a.abs();
	let abs_b = b.abs();
	let diff = (a - b).abs();

	if a == b { // Shortcut, handles infinities.
		true
	} else if a == 0.0 || b == 0.0 || diff < f32::MIN_POSITIVE {
		// a or b is zero or both are extremely close to it
		// relative error is less meaningful here
		diff < (f32::EPSILON * f32::MIN_POSITIVE)
	} else { // Use relative error.
		(diff / f32::min(abs_a + abs_b, f32::MAX)) < f32::EPSILON
	}
}


/// Returns the given value clamped between the provided bounds.
/// 
/// # Examples
///
/// ```rust
/// # use rampeditor::utilities::{clamped, nearly_equal};
/// let a = clamped(7.6, 4.3, 7.4);
///
/// assert!(nearly_equal(a, 7.4));
/// ```
pub fn clamped(value: f32, lower_bound: f32, upper_bound: f32) -> f32 {
	assert!(lower_bound <= upper_bound);
	if value < lower_bound {
		lower_bound
	} else if value > upper_bound {
		upper_bound
	} else {
		value
	}
}


/// Performs a linear interpolation between `start` and `end`, returning the 
/// value located at the ratio given by `amount`, which is clamped between 0 and
/// 1. 
///
/// # Examples
///
/// ```rust
/// # use rampeditor::utilities::lerp_u8;
/// let a = lerp_u8(50, 100, 0.5);
///
/// assert_eq!(a, 75); // a is 50% between 50 and 100.
/// ```
///
/// ```rust
/// # use rampeditor::utilities::lerp_u8;
/// let a = lerp_u8(15, 5, 0.2);
/// let b = lerp_u8(5, 15, 0.8);
///
/// assert_eq!(a, b); // Reversed argument order inverts the ratio.
/// ```
pub fn lerp_u8(start: u8, end:u8, amount: f32) -> u8 {
	let a = if start > end {
		1.0 - clamped(amount, 0.0, 1.0)
	} else {
		clamped(amount, 0.0, 1.0)
	};

	let s = if start > end {end} else {start};
	let e = if start > end {start} else {end};
	(((e-s) as f32) * a) as u8 + s
}


/// Performs a linear interpolation between `start` and `end`, returning the 
/// value located at the ratio given by `amount`, which is clamped between 0 and
/// 1. 
///
/// # Examples
///
/// ```rust
/// # use rampeditor::utilities::{lerp_f32, nearly_equal};
/// let a = lerp_f32(50.0, 100.0, 0.5);
///
/// assert!(nearly_equal(a, 75.0)); // a is 50% between 50 and 100.
/// ```
///
/// ```rust
/// # use rampeditor::utilities::{lerp_f32, nearly_equal};
/// let a = lerp_f32(15.0, 5.0, 0.2);
/// let b = lerp_f32(5.0, 15.0, 0.8);
///
/// assert!(nearly_equal(a, b)); // Reversed argument order inverts the ratio.
/// ```
pub fn lerp_f32(start: f32, end:f32, amount: f32) -> f32 {
	let a = if start > end {
		1.0 - clamped(amount, 0.0, 1.0)
	} else {
		clamped(amount, 0.0, 1.0)
	};

	let s = if start > end {end} else {start};
	let e = if start > end {start} else {end};
	(((e-s) as f32) * a) as f32 + s
}
