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
//! Defines structured Palette objects for storing and generating colors.
//!
//! The palette acts as a tree-like structure that acts as a collection of 
//! 'Slots' into which color elements are placed. Rgb elements will then 
//! lazily generate a color when queried. This allows for the construction of 
//! dynamic palette structures that can generate related colors based off of a 
//! small subset of 'control' colors.
//!
//! More practically, `Slot`s are identified by `Address`, and each slot 
//! contains a single `ColorElement`, which will generate a `Rgb` when either
//! the Slot's or ColorElement's `get_color` method is called. ColorElements are
//! categorized by 'order', which denotes the number of dependencies needed to
//! generate a color. For example, a second order element is dependent upon two
//! other colors, while a zeroth order color element is simply a color. These
//! dependencies are expressed through references to other slots in the palette.
//!
////////////////////////////////////////////////////////////////////////////////

#[warn(missing_docs)]
pub mod error;
#[warn(missing_docs)]
pub mod element;
#[warn(missing_docs)]
pub mod data;
#[warn(missing_docs)]
pub mod operation;
// #[warn(missing_docs)]
// pub mod format;

// pub use palette::format::{Palette, PaletteExtensions, BasicPalette, ZplPalette};
pub use palette::error::{Error, Result};
pub use palette::operation::{
	InsertColor,
	RemoveElement,
	CopyColor,
	InsertWatcher,
	SequenceOperation,
	RepeatOperation,
	InsertRamp,
};

use palette::operation::PaletteOperation;
use palette;
use address::Address;
use color::Rgb;

use std::fmt;
use std::io::{Write, Read};
use std::io;

////////////////////////////////////////////////////////////////////////////////
// Palette
////////////////////////////////////////////////////////////////////////////////
/// Specifies the interface for using a specific palette format.
pub trait Palette : fmt::Debug {
	/// Creates a new palette with the given name.
	fn new<S>(name: S) -> Self where S: Into<String>;

	/// Returns the color at the given address, or None if the slot is empty.
	fn get_color(&self, address: Address) -> Option<Rgb>;

	/// Returns the number of elements in the palette.
	fn len(&self) -> usize;

	/// Applies the given operation to the palette. Usually, this will just 
	/// defer to the PaletteOperation's apply method, but this could also 
	/// provide extra functionality such as undo/redo and format-specific 
	/// checks.
	fn apply_operation(
		&mut self, 
		mut operation: Box<PaletteOperation>) 
		-> palette::Result<()>;

	/// Reverses the most recently applied operation.
	fn undo(&mut self) -> palette::Result<()> {
		panic!("operation not supported")
	}

	/// Reverses the most recently applied undo operation.
	fn redo(&mut self) -> palette::Result<()> {
		panic!("operation not supported")
	}

	/// Writes the palette to the given buffer.
	#[allow(unused_variables)]
	fn write_palette<W>(&self, out_buf: &mut W) -> io::Result<()> 
		where W: io::Write
	{
		panic!("operation not supported")
	}

	/// Reads a palette from the given buffer.
	#[allow(unused_variables)]
	fn read_palette<R>(in_buf: &R) -> io::Result<Self>
		where R: io::Read, Self: Sized
	{
		panic!("operation not supported")
	}
}