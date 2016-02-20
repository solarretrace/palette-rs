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
//! Provides components for interacting with the default palette format.
//!
////////////////////////////////////////////////////////////////////////////////
use palette::format::Palette;
use palette::PaletteData;
use address;

use std::fmt;
use std::result;

////////////////////////////////////////////////////////////////////////////////
// DefaultPalette
////////////////////////////////////////////////////////////////////////////////
/// The default palette format with no special configuration.
#[derive(Debug)]
pub struct DefaultPalette {
	core: PaletteData,
}

impl Palette for DefaultPalette {

	fn new<S>(name: S) -> Self where S: Into<String> {
		let mut pal = DefaultPalette {core: Default::default()};
		pal.core.set_label(address::Select::All, "DefaultPalette 1.0.0");
		pal.core.set_name(address::Select::All, name.into());
		pal
	}
}

impl fmt::Display for DefaultPalette {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		write!(f, "{} {}",
			self.core.get_label(address::Select::All).unwrap_or(""),
			self.core
		)
	}
}