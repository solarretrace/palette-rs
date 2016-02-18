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
//! Provides components for interacting with the ZPL palette format.
//!
////////////////////////////////////////////////////////////////////////////////
use super::format::PaletteFormat;

use palette::PaletteBuilder;

use std::fmt;

// The ZPL format was built for version 2.50 build 24 of Zelda Classic, and may
// not work on versions 1.92 or older.
const ZPL_HEADER : [u8;12] = [
	0x43, 0x53, 0x45, 0x54, 
	0x04, 0x00, 0x01, 0x00, 
	0x9c, 0x0d, 0x05, 0x00
];

const ZPL_FOOTER_A : [u8;4] = [
	0x5a, 0x00, 0x00, 0x00
];

const ZPL_FOOTER_B : [u8;4] = [ // x 109
	0x00, 0x00, 0x00, 0x00
];

const ZPL_FOOTER_C : [u8;20] = [
	0x00, 0x00, 0x00, 0x36, 
	0x00, 0x00, 0x4e, 0x00, 
	0x00, 0x14, 0x00, 0x00, 
	0x36, 0x00, 0x00, 0x4e,
	0x00, 0x00, 0x14, 0x00
];

const ZPL_FOOTER_D : [u8;4] = [ // x 79
	0x00, 0x00, 0x00, 0x00
];

const ZPL_FOOTER_E : [u8;36] = [
	0x22, 0x00, 0x00, 0x66, 
	0x00, 0x00, 0x5a, 0x00, 
	0x00, 0x22, 0x00, 0x00, 
	0x86, 0x00, 0x00, 0x3c,
	0x00, 0x00, 0x22, 0x00, 
	0x00, 0x86, 0x00, 0x00, 
	0x3c, 0x00, 0x00, 0x20, 
	0x30, 0x40, 0x3f, 0x3f, 
	0x3f, 0x07, 0x07, 0x07
];

////////////////////////////////////////////////////////////////////////////////
// ZplFormat
////////////////////////////////////////////////////////////////////////////////
/// The default palette format with no special configuration.
pub struct ZplFormat;

/// A reference to a small pallete PaletteFormat for configuring palettes.
pub const ZPL_FORMAT: &'static ZplFormat = &ZPL_FORMAT_INSTANCE;
const ZPL_FORMAT_INSTANCE: ZplFormat = ZplFormat;


impl PaletteFormat for ZplFormat {
	fn configure(&self, builder: PaletteBuilder) -> PaletteBuilder {
		builder
			.with_page_count(8)
			.with_line_count(16)
			.with_column_count(16)
	}
}

impl fmt::Debug for ZplFormat {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "ZplFormat 1.0.0")
	}
}

