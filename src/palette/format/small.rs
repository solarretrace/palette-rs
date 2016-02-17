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
//! Provides components for interacting with a small palette format.
//!
////////////////////////////////////////////////////////////////////////////////
use super::format::PaletteFormat;

use palette::PaletteBuilder;

use std::fmt;



////////////////////////////////////////////////////////////////////////////////
// SmallFormat
////////////////////////////////////////////////////////////////////////////////
/// The default palette format with no special configuration.
pub struct SmallFormat;

/// A reference to a small pallete PaletteFormat for configuring palettes.
pub const SMALL_FORMAT: &'static SmallFormat = &SMALL_FORMAT_INSTANCE;
const SMALL_FORMAT_INSTANCE: SmallFormat = SmallFormat;


impl PaletteFormat for SmallFormat {
	fn get_name(&self) -> &'static str {"SmallFormat"}
	fn get_version(&self) -> (u8, u8, u8) {(0, 1, 0)}
	fn configure(&self, builder: PaletteBuilder) -> PaletteBuilder {
		builder
			.with_page_count(8)
			.with_line_count(16)
			.with_column_count(16)
	}
}

impl fmt::Debug for SmallFormat {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, 
			"SmallFormat{{ name: {:?}, version: {:?} }}", 
			self.get_name(),
			self.get_version())
	}
}