
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
//! Defines functionality common to all operation modules.
//!
////////////////////////////////////////////////////////////////////////////////
use palette::data::PaletteData;
use palette::history::HistoryEntry;
use palette::operations::Undo;
use palette::element::Slot;
use palette::{Error, Result};
use palette;
use address::Address;

use std::fmt;
use std::rc::{Rc, Weak};


/// Gets a weak reference to the source element located at the given address 
/// from the given palette. If the slot is empty, it will be created if 
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


////////////////////////////////////////////////////////////////////////////////
// PaletteOperation
////////////////////////////////////////////////////////////////////////////////
/// Provides the methods for modifying palettes.
pub trait PaletteOperation: fmt::Debug {
	/// Applies the operation to the given palette.
	fn apply(self, data: &mut PaletteData) -> palette::Result<HistoryEntry>;
}