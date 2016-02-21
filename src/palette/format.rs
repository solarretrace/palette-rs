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
use palette::PaletteData;
use address::Group;

use std::fmt;

use std::io;
use std::io::{Result, Write, Read};

////////////////////////////////////////////////////////////////////////////////
// PalettePalette
////////////////////////////////////////////////////////////////////////////////
/// Specifies the interface for using a specific palette format.
pub trait Palette : fmt::Debug {
	/// Creates a new palette with the given name.
	fn new<S>(name: S) -> Self where S: Into<String>, Self: Sized;

	/// Called before an element is added to a new page in the palette. The 
	/// expectation is that this will add the appropriate meta data to the 
	/// palette. This will be called before the prepare_new_line function is 
	/// called.
	#[allow(unused_variables)]
	fn prepare_new_page(&self, palette: &mut PaletteData, page: Group) {}

	/// Called before an element is added to a new line in the palette. The 
	/// expectation is that this will add the appropriate meta data to the 
	/// palette.
	#[allow(unused_variables)]
	fn prepare_new_line(&self, palette: &mut PaletteData, line: Group) {}

	/// Writes the palette to the given buffer.
	#[allow(unused_variables)]
	fn write_palette<W>(&self, out_buf: &mut W) -> io::Result<()> 
		where W: io::Write
	{
		unimplemented!()
	}

	/// Reads a palette from the given buffer.
	#[allow(unused_variables)]
	fn read_palette<R>(in_buf: &R) -> io::Result<Self>
		where R: io::Read, Self: Sized
	{
		unimplemented!()
	}
}

