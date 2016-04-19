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


////////////////////////////////////////////////////////////////////////////////
// Format
////////////////////////////////////////////////////////////////////////////////
/// The supported palette formats.
#[derive(Debug, Clone, Copy)]
pub enum Format {
	/// The default palette format; provides no special behaviors or 
	/// restrictions.
	Default,
	/// The ZPL palette format. 
	Zpl,
}


impl Format {
	/// Called when a new palette is created. Initializes the palette data.
	#[allow(unused_variables)]
	pub fn initialize(&self, palette: &mut Palette)  {
		match *self {
			Format::Zpl => zpl::initialize(palette),
			_ => (),
		}
	}

	/// The function to call when a new page is created.
	#[allow(unused_variables)]
	pub fn prepare_new_page(&self, data: &mut PaletteOperationData, group: Group) {
		match *self {
			Format::Zpl => zpl::prepare_new_page(data, group),
			_ => (),
		}
	}

	/// The function to call when a new line is created.
	#[allow(unused_variables)]
	pub fn prepare_new_line(&self, data: &mut PaletteOperationData, group: Group) {
		match *self {
			Format::Zpl => zpl::prepare_new_line(data, group),
			_ => (),
		}
	}

	/// Applies the given operation to the palette. Usually, this will just 
	/// defer to the PaletteOperation's apply method, but this could also 
	/// provide extra functionality such as undo/redo and format-specific 
	/// checks.
	#[allow(unused_variables)]
	pub fn apply_operation(
		&self, 
		palette: &mut Palette, 
		mut operation: Box<PaletteOperation>) 
		-> palette::Result<()>
	{
		default::apply_operation(palette, operation)
	}

	/// Reverses the most recently applied operation.
	#[allow(unused_variables)]
	pub fn undo(&self, palette: &mut Palette) -> palette::Result<()> {
		default::undo(palette)
	}

	/// Reverses the most recently applied undo operation.
	#[allow(unused_variables)]
	pub fn redo(&self, palette: &mut Palette) -> palette::Result<()> {
		default::redo(palette)
	}
}


