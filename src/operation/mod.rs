// The MIT License (MIT)
// 
// Copyright (c) 2017 Skylor R. Schermer
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
//! The `operation` module provides implementations of the high-level,
//! reversable operations that may be applied to a `Palette`.
//!
////////////////////////////////////////////////////////////////////////////////


// Sumbodules.
#[warn(missing_docs)]
mod basic;
#[warn(missing_docs)]
mod combine;
#[warn(missing_docs)]
mod undo;

// Submodule re-exports.
pub use self::basic::{
	InsertCell,
	DeleteCell,
};
pub use self::combine::{
	Repeat,
	Sequence,
};
pub use self::undo::Undo;

// Local imports.
use address::Address;
use cell::Cell;
use data::Data;
use expression::Expression;
use result::{Error, Result};

// Standard imports.
use std::fmt;
use std::rc::{Rc, Weak};
use std::mem;


/// Returns a weak reference to the source element located at the given address 
/// in the given palette. If the cell is empty, it will be created if 
/// `make_sources` is true. If the source is created, its creation will be
/// logged in the provided `Undo` operation.
pub(crate) fn source(
	data: &mut Data, 
	address: Address, 
	make_sources: bool,
	undo: &mut Undo) 
	-> Result<Weak<Cell>>
{
	if let Some(cell) = data.cell(address) {
		Ok(Rc::downgrade(&cell))
	} else if make_sources {
		let cell = Rc::downgrade(&data.create_cell(address)?);
		undo.record(address, None);
		Ok(cell)
	} else {
		Err(Error::InvalidAddress(address))
	}
}

/// Returns a reference to the target element located at the given address in
/// the given palette. If the cell is empty, it will be created.
pub(crate) fn target(
	data: &mut Data, 
	address: Address, 
	undo: &mut Undo)
	-> Result<Rc<Cell>>
{
	if let Some(cell) = data.cell(address) {
		Ok(cell)
	} else {
		let cell = data.create_cell(address)?;
		undo.record(address, None);
		Ok(cell)
	}
}

/// Stores the given Expression in the cell at the given address in the given 
/// palette. If the cell is empty, it will be created.
pub(crate) fn set_target(
	data: &mut Data,
	address: Address,
	new_element: Expression,
	undo: &mut Undo)
	-> Result<()>
{
	// Get the target cell.
	let target = target(data, address, undo)?;

	// Insert new element into palette.
	let cur = mem::replace(&mut *target.borrow_mut(), new_element);
	undo.record(address, Some(cur));
	Ok(())
}



////////////////////////////////////////////////////////////////////////////////
// PaletteOperation
////////////////////////////////////////////////////////////////////////////////
/// Provides the methods for modifying palettes.
pub trait PaletteOperation: fmt::Debug {
	/// Returns information about the operation.
	fn info(&self) -> OperationInfo;

	/// Applies the operation to the given palette.
	fn apply(&mut self, data: &mut Data) 
		-> Result<HistoryEntry>;
}



////////////////////////////////////////////////////////////////////////////////
// OperationHistory
////////////////////////////////////////////////////////////////////////////////
/// Maintains a history of operations applied to a palette and their associated
/// undo operations.
#[derive(Debug, Default)]
pub struct OperationHistory {
	/// The record of available undos.
	pub undo_entries: Vec<HistoryEntry>,
	/// The record of available redos.
	pub redo_entries: Vec<HistoryEntry>,
}




////////////////////////////////////////////////////////////////////////////////
// HistoryEntry
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single entry in the operation history.
#[derive(Debug)]
pub struct HistoryEntry {
	/// Information about the operation that was applied to the palette.
	pub info: OperationInfo,
	/// The operation that undoes the applied operation.
	pub undo: Box<PaletteOperation>,
}



////////////////////////////////////////////////////////////////////////////////
// OperationInfo
////////////////////////////////////////////////////////////////////////////////
/// Describes an applied operation.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone)]
pub struct OperationInfo {
	/// The name of the operation.
	pub name: &'static str,
	/// The details of the operation.
	pub details: Option<String>,
}


