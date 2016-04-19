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
//! Provides format-dependent palette operations.
//!
////////////////////////////////////////////////////////////////////////////////

// Module declarations.
#[warn(missing_docs)]
#[allow(dead_code)]
pub mod zpl;
#[warn(missing_docs)]
pub mod default;

// Module imports.
use palette::Palette;
use palette::data::PaletteOperationData;
use palette::operation::PaletteOperation;
use palette::format::default::*;
use palette::format::zpl::*;
use palette;
use address::Group;

use std::io;

////////////////////////////////////////////////////////////////////////////////
// Format
////////////////////////////////////////////////////////////////////////////////
/// An enum of the supported palette formats.
#[derive(Debug, Clone, Copy)]
pub enum Format {
	/// The default palette format; provides no special behaviors or 
	/// restrictions.
	Default,
	/// The ZPL palette format. Lines are 15 columns wide, and there are 16 
	/// lines per page, for 211 pages. The names of lines and pages are 
	/// auto-generated.
	Zpl,
}


impl Format {
	/// Called when a new palette is created. Initializes the palette data.
	pub fn initialize(self, data: &mut PaletteOperationData)  {
		match self {
			Format::Zpl => zpl::initialize(data),
			_ => (),
		}
	}

	/// Called when a new page is created.
	pub fn prepare_new_page(
		self, data: 
		&mut PaletteOperationData, 
		group: Group) 
	{
		match self {
			Format::Zpl => zpl::prepare_new_page(data, group),
			_ => (),
		}
	}

	/// Called when a new line is created.
	pub fn prepare_new_line(
		self, 
		data: &mut PaletteOperationData, 
		group: Group) 
	{
		match self {
			Format::Zpl => zpl::prepare_new_line(data, group),
			_ => (),
		}
	}

	/// Applies the given operation to the palette. 
	pub fn apply_operation(
		self, 
		palette: &mut Palette, 
		operation: Box<PaletteOperation>) 
		-> palette::Result<()>
	{
		default::apply_operation(palette, operation)
	}

	/// Reverses the most recently applied operation.
	pub fn undo(self, palette: &mut Palette) -> palette::Result<()> {
		default::undo(palette)
	}

	/// Reverses the most recently applied undo operation.
	pub fn redo(self, palette: &mut Palette) -> palette::Result<()> {
		default::redo(palette)
	}

	/// Writes the palette to the given buffer.
	#[allow(unused_variables)]
	pub fn write_palette<W>(self, palette: &Palette, out_buf: &mut W) -> io::Result<()> 
		where W: io::Write
	{
		unimplemented!()
	}

	/// Reads a palette from the given buffer.
	#[allow(unused_variables)]
	pub fn read_palette<R>(self, in_buf: &mut R) -> io::Result<()> 
		where R: io::Read
	{
		unimplemented!()
	}
}


