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
//! Provides a set of interfaces and implementations for converting palettes 
//! between different formats.
//!
////////////////////////////////////////////////////////////////////////////////

// Module declarations.
// #[warn(missing_docs)]
// pub mod basic;
// #[warn(missing_docs)]
// #[allow(dead_code)]
// pub mod zpl;

// Re-exports.
// pub use palette::format::basic::BasicPalette;
// pub use palette::format::zpl::ZplPalette;

// Module imports.
use palette::data::PaletteOperationData;
use address::Group;

use std::fmt;
use std::result;

/// A trait for defining palette formats.
pub trait PaletteFormat: fmt::Debug {
	/// The function to call when a new page is created.
	fn prepare_new_page(data: &mut PaletteOperationData, group: Group) 
		where Self: Sized; // Required to provide a receiver for the method.
	/// The function to call when a new line is created.
	fn prepare_new_line(data: &mut PaletteOperationData, group: Group) 
		where Self: Sized; // Required to provide a receiver for the method.
}


/// A palette format with no restrictions or special behaviors.
pub struct DefaultPaletteFormat;

#[allow(unused_variables)]
impl PaletteFormat for DefaultPaletteFormat {
	fn prepare_new_page(data: &mut PaletteOperationData, group: Group) {}
	fn prepare_new_line(data: &mut PaletteOperationData, group: Group) {}
}

impl fmt::Debug for DefaultPaletteFormat {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		write!(f, "DefaultPaletteFormat")
	}
}

/// A palette format with no restrictions or special behaviors.
pub static DEFAULT_PALETTE_FORMAT: DefaultPaletteFormat = DefaultPaletteFormat;