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
//! Provides common definitions for format specifiers.
//!
////////////////////////////////////////////////////////////////////////////////
use palette::{Palette, PaletteBuilder};
use address;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////
// PaletteFormat
////////////////////////////////////////////////////////////////////////////////
/// Specifies the interface for using a specific palette format.
pub trait PaletteFormat : fmt::Debug {	
	/// Returns a configured palette builder that will create a valid palette 
	/// for this format.
	#[allow(unused_mut)]
	fn configure(&self, mut builder: PaletteBuilder) -> PaletteBuilder {
		builder
	}

	/// Called after the palette has been configured and created.
	#[allow(unused_variables)]
	fn prepare_new_palette(&self, palette: &mut Palette) {}

	/// Called before an element is added to a new page in the palette. The 
	/// expectation is that this will add the appropriate meta data to the 
	/// palette. This will be called before the prepare_new_line function is 
	/// called.
	#[allow(unused_variables)]
	fn prepare_new_page(&self, palette: &mut Palette, page: address::Select) {}

	/// Called before an element is added to a new line in the palette. The 
	/// expectation is that this will add the appropriate meta data to the 
	/// palette.
	#[allow(unused_variables)]
	fn prepare_new_line(&self, palette: &mut Palette, line: address::Select) {}
}


////////////////////////////////////////////////////////////////////////////////
// ZplFormat
////////////////////////////////////////////////////////////////////////////////
/// The default palette format with no special configuration.
#[derive(Debug)]
pub struct DefaultPaletteFormat;

impl PaletteFormat for DefaultPaletteFormat {}

/// A reference to a small pallete PaletteFormat for configuring palettes.
pub const DEFAULT_FORMAT: &'static DefaultPaletteFormat 
	= &DEFAULT_FORMAT_INSTANCE;

const DEFAULT_FORMAT_INSTANCE: DefaultPaletteFormat = DefaultPaletteFormat;