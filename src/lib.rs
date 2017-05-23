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
//! Provides structured `Palette` objects for storing and generating colors.
//!
//! The palette acts as a tree-like structure that acts as a collection of 
//! 'Cell's into which color elements are placed. Color elements will then 
//! lazily generate a color when queried. This allows for the construction of 
//! dynamic palette structures that can generate related colors based off of a 
//! small subset of 'control' colors.
//!
//! More practically, `Cell`s are identified by `Address`, and each cell 
//! contains a single `Expression`, which will generate a `Color` when either
//! the Cell's or Expression's `color` method is called. Expressions are
//! categorized by 'order', which denotes the number of dependencies needed to
//! generate a color. For example, a second order element is dependent upon two
//! other colors, while a zeroth order color element is simply a color. These
//! dependencies are expressed through references to other cells in the palette.
//!
////////////////////////////////////////////////////////////////////////////////

extern crate color;
extern crate interval;

// Submodules.
#[warn(missing_docs)]
pub mod address;
#[warn(missing_docs)]
pub mod cell;
#[warn(missing_docs)]
pub mod data;
#[warn(missing_docs)]
pub mod expression;
#[warn(missing_docs)]
pub mod format;
#[warn(missing_docs)]
pub mod operation;
#[warn(missing_docs)]
pub mod result;
#[warn(missing_docs)]
pub mod utilities;



// Non-local re-exports.
pub use color::Color;

// Submodule re-exports
pub use address::{
	Address,
	Reference,
};
pub use expression::Expression;
pub use format::Format;


// Local imports.
use data::Data;
use operation::{PaletteOperation, OperationHistory};
use result::Result;

// Standard imports.
use std::fmt;



////////////////////////////////////////////////////////////////////////////////
// Palette
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single color palette.
#[derive(Debug)]
pub struct Palette {
	/// The `Palette`'s operation-relevant data.
	data: Data,

	/// The operation undo and redo history.
	operation_history: Option<OperationHistory>,
	
	/// The palette format.
	format: Format,
}


impl Palette {
	/// Creates a new `Palette` with the given name.
	pub fn new<S>(name: S, format: Format, history: bool) 
		-> Self where S: Into<String> 
	{
		let mut pal = Palette {
			data: Default::default(),
			operation_history: if history {
					Some(Default::default())
				} else {
				    None
				},
			format: format,
		};
		
		pal.data.set_name(Reference::all(), name.into());
		format.initialize(&mut pal.data);
		pal
	}

	/// Returns the number of color `Cell`s in the `Palette`.
	pub fn len(&self) -> usize {
		self.data.len()
	}

	/// Returns whether the `Palette` contains any color `Cell`s.
	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}

	/// Returns the total number of history entries recorded.
	pub fn history_len(&self) -> (usize, usize) {
		if let Some(ref history) = self.operation_history {
			(history.undo_entries.len(), history.redo_entries.len())
		} else {
			(0, 0)
		}
	}

	/// Returns whether the `Palette` contains any history entries.
	pub fn history_is_empty(&self) -> bool {
		if let Some(ref history) = self.operation_history {
			history.undo_entries.is_empty() && history.redo_entries.is_empty()
		} else {
			false
		}
	}

	/// Returns the color at the given address, or None if the cell is empty.
	pub fn color(&self, address: Address) -> Option<Color> {
		self.data.cell(address).and_then(|cell| cell.color())
	}


	/// Applies the given operation to the `Palette`. Usually, this will just 
	/// defer to the `PaletteOperation`'s apply method, but this could also 
	/// provide extra functionality such as undo/redo and format-specific 
	/// checks.
	#[allow(unused_variables)]
	pub fn apply(
		&mut self, 
		operation: Box<PaletteOperation>)
		-> Result<()> 
	{
		self.format.apply_operation(self, operation)
	}

	/// Reverses the most recently applied operation.
	#[allow(unused_variables)]
	pub fn undo(&mut self) -> Result<()> {
		self.format.undo(self)
	}

	/// Reverses the most recently applied undo operation.
	#[allow(unused_variables)]
	pub fn redo(&mut self) -> Result<()> {
		self.format.redo(self)
	}
}



// Default is empty `Palette` with default format.
impl Default for Palette {
	fn default() -> Self {
		Palette {
			data: Default::default(),
			operation_history: None,
			format: Format::Default,
		}
	}
}



// Display `Palette` in readable format.
impl fmt::Display for Palette {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Format: {:?}, History: {:?}\n{}",
			self.format,
			self.history_len(),
			self.data)
	}
}