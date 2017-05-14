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
//! 'Slots' into which color elements are placed. Color elements will then 
//! lazily generate a color when queried. This allows for the construction of 
//! dynamic palette structures that can generate related colors based off of a 
//! small subset of 'control' colors.
//!
//! More practically, `Slot`s are identified by `Address`, and each slot 
//! contains a single `ColorElement`, which will generate a `Color` when either
//! the Slot's or ColorElement's `get_color` method is called. ColorElements are
//! categorized by 'order', which denotes the number of dependencies needed to
//! generate a color. For example, a second order element is dependent upon two
//! other colors, while a zeroth order color element is simply a color. These
//! dependencies are expressed through references to other slots in the palette.
//!
////////////////////////////////////////////////////////////////////////////////

// Module declarations.
#[warn(missing_docs)]
pub mod error;
#[warn(missing_docs)]
pub mod element;
#[warn(missing_docs)]
pub mod data;
#[warn(missing_docs)]
pub mod operation;
// #[warn(missing_docs)]
pub mod format;

// Re-exports.
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

// Module imports.
use palette::operation::{OperationHistory, PaletteOperation};
use palette::format::Format;
use palette::data::PaletteOperationData;
use palette;
use address::{Address, Group};
use color::Color;

use std::fmt;
use std::result;



////////////////////////////////////////////////////////////////////////////////
// Palette
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single palette.
#[derive(Debug)]
pub struct Palette {
	/// The palette's operation-relevant data.
	data: PaletteOperationData,
	/// The operation undo and redo history.
	operation_history: Option<OperationHistory>,
	/// The palette format.
	format: Format,
}


impl Palette {
	/// Creates a new palette with the given name.
	pub fn new<S>(name: S, format: Format, history: bool) 
		-> Self where S: Into<String> 
	{
		let mut pal = Palette {
			data: Default::default(),
			operation_history: if history {
					Some(OperationHistory::new())
				} else {
				    None
				},
			format: format,
		};
		
		pal.data.set_name(Group::All, name.into());
		format.initialize(&mut pal.data);
		pal
	}

	/// Returns the total number of history entries recorded.
	pub fn history_len(&self) -> (usize, usize) {
		if let Some(ref history) = self.operation_history {
			(history.undo_entries.len(), history.redo_entries.len())
		} else {
			(0, 0)
		}
	}

	/// Returns the color at the given address, or None if the slot is empty.
	pub fn get_color(&self, address: Address) -> Option<Color> {
		self.data.get_slot(address).and_then(|slot| slot.get_color())
	}

	/// Returns the number of elements in the palette.
	pub fn len(&self) -> usize {
		self.data.len()
	}

	/// Applies the given operation to the palette. Usually, this will just 
	/// defer to the PaletteOperation's apply method, but this could also 
	/// provide extra functionality such as undo/redo and format-specific 
	/// checks.
	#[allow(unused_variables)]
	pub fn apply(
		&mut self, 
		operation: Box<PaletteOperation>)
		-> palette::Result<()> 
	{
		self.format.apply_operation(self, operation)
	}

	/// Reverses the most recently applied operation.
	#[allow(unused_variables)]
	pub fn undo(&mut self) -> palette::Result<()> {
		self.format.undo(self)
	}

	/// Reverses the most recently applied undo operation.
	#[allow(unused_variables)]
	pub fn redo(&mut self) -> palette::Result<()> {
		self.format.redo(self)
	}
}


impl Default for Palette {
	fn default() -> Self {
		Palette {
			data: Default::default(),
			operation_history: None,
			format: Format::Default,
		}
	}
}


impl fmt::Display for Palette {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		write!(f, "Format: {:?}, History: {:?}\n{}",
			self.format,
			self.history_len(),
			self.data)
	}
}