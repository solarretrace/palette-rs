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
//! A library providing color-ramp editing tools.
//!
////////////////////////////////////////////////////////////////////////////////

// Module declarations.
#[warn(missing_docs)]
pub mod palette;
#[warn(missing_docs)]
pub mod address;
#[warn(missing_docs)]
pub mod utilities;
#[warn(missing_docs)]
pub mod interval;
#[warn(missing_docs)]
pub mod color;
#[warn(missing_docs)]
pub mod gui;

// Re-exports.
pub use color::{Color, Cmyk, Hsl, Hsv, Rgb, Xyz};
pub use palette::Palette;
pub use address::{Address, Group, Selection};
pub use interval::{Bound, Interval};
pub use palette::operation::{
	InsertColor,
	RemoveElement,
	CopyColor,
	InsertWatcher,
	SequenceOperation,
	RepeatOperation,
	InsertRamp,
};
