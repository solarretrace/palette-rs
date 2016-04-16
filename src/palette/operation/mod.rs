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
//! Contains definitions for various palette editing operations.
//!
////////////////////////////////////////////////////////////////////////////////

#[warn(missing_docs)]
pub mod arrange;
#[warn(missing_docs)]
pub mod duplicate;
#[warn(missing_docs)]
pub mod ramp;
#[warn(missing_docs)]
pub mod sequence;
#[warn(missing_docs)]
pub mod simple;
#[warn(missing_docs)]
pub mod undo;

pub use palette::operation::arrange::*;
pub use palette::operation::duplicate::*;
pub use palette::operation::ramp::*;
pub use palette::operation::sequence::*;
pub use palette::operation::simple::*;
pub use palette::operation::undo::*;

use palette::data::PaletteData;
use palette::element::{Slot, ColorElement};
use palette::{Error, Result};
use palette;
use address::Address;

use std::fmt;
use std::rc::{Rc, Weak};
use std::mem;

/// Returns a weak reference to the source element located at the given address 
/// in the given palette. If the slot is empty, it will be created if 
/// make_sources is true. If the source is created, its creation will be logged 
/// in the provided Undo operation.
pub fn get_source(
	data: &mut PaletteData, 
	address: Address, 
	make_sources: bool,
	undo: &mut Undo) 
	-> Result<Weak<Slot>>
{
	if let Some(slot) = data.get_slot(address) {
		Ok(Rc::downgrade(&slot))
	} else if make_sources {
		let slot = Rc::downgrade(&try!(data.create_slot(address)));
		undo.record(address, None);
		Ok(slot)
	} else {
		Err(Error::InvalidAddress(address))
	}
}

/// Returns a reference to the target element located at the given address in
/// the given palette. If the slot is empty, it will be created.
pub fn get_target(
	data: &mut PaletteData, 
	address: Address, 
	undo: &mut Undo)
	-> Result<Rc<Slot>>
{
	if let Some(slot) = data.get_slot(address) {
		Ok(slot)
	} else {
		let slot = try!(data.create_slot(address));
		undo.record(address, None);
		Ok(slot)
	}
}

/// Stores the given ColorElement in the slot at the given address in the given 
/// palette. If the slot is empty, it will be created.
pub fn set_target(
	data: &mut PaletteData,
	address: Address,
	new_element: ColorElement,
	undo: &mut Undo)
	-> Result<()>
{
	// Get the target slot.
	let target = try!(get_target(data, address, undo));

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
	fn get_info(&self) -> OperationInfo;

	/// Applies the operation to the given palette.
	fn apply(&mut self, data: &mut PaletteData) 
		-> palette::Result<HistoryEntry>;
}




////////////////////////////////////////////////////////////////////////////////
// OperationHistory
////////////////////////////////////////////////////////////////////////////////
/// Maintains a history of operations applied to a palette and their associated
/// undo operations.
#[derive(Debug)]
pub struct OperationHistory {
	/// The record of available undos.
	pub undo_entries: Vec<HistoryEntry>,
	/// The record of available redos.
	pub redo_entries: Vec<HistoryEntry>,
}


impl OperationHistory {
	/// Creates a new, empty OperationHistory
	pub fn new() -> OperationHistory {
		OperationHistory {
			undo_entries: Vec::new(),
			redo_entries: Vec::new(),
		}
	}
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