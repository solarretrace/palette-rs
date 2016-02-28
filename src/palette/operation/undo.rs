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
//! Defines an undo operation to be returned by other operations.
//!
////////////////////////////////////////////////////////////////////////////////
use super::common::PaletteOperation;

use palette::Result;
use palette::data::PaletteData;
use palette::element::ColorElement;
use palette::history::{HistoryEntry, EntryInfo};
use address::Address;

use std::mem;
use std::collections::HashMap;


////////////////////////////////////////////////////////////////////////////////
// Undo
////////////////////////////////////////////////////////////////////////////////
/// Restores a saved set of elements in the palette. 
/// 
/// The Undo operations stores ColorElements using a HashMap, which means it can
/// only store one entry for each address. An create operation will have
/// priority over any other change recorded. In otherwords, if there is an
/// "address: None" entry in the Undo,  nothing will overwrite it. This ensures
/// that the element at that address  will be deleted if the Undo operation is
/// applied later.
#[derive(Debug)]
pub struct Undo {
	/// The operation being undone.
	undoing: Option<Box<PaletteOperation>>,
	/// The ColorElements to restore when applying the Undo.
	saved: HashMap<Address, Option<ColorElement>>,
}

impl Undo {
	/// Creates a new Undo operation.
	#[inline]
	fn new() -> Undo {
		Undo {
			undoing: None,
			saved: Default::default(),
		}
	}

	/// Creates a new Undo operation for the given operation.
	#[inline]
	pub fn new_for<O>(operation: &O) -> Undo 
		where O: PaletteOperation + Clone + 'static
	{
		Undo {
			undoing: Some(Box::new(operation.clone())),
			saved: Default::default(),
		}
	}

	/// Records an element change to be replayed by the Undo operation.
	#[inline]
	pub fn record(&mut self, address: Address, element: Option<ColorElement>) {
		if self.saved.get(&address).map_or(true, |e| !e.is_none()) {
			self.saved.insert(address, element);
		}
	}

}


impl PaletteOperation for Undo {
	fn apply(self, data: &mut PaletteData) -> Result<HistoryEntry> {
		let mut redo = Undo::new();

		for (address, item) in self.saved {
			match (item.is_some(), data.get_slot(address).is_some()) {

				(true, true) => { // The slot was modified.
					let elem = item.unwrap();
					let slot = data.get_slot(address).unwrap();
					let cur = mem::replace(&mut *slot.borrow_mut(), elem);
					redo.record(address, Some(cur));
					continue;
				},

				(true, false) => { // The slot was deleted.
					let elem = item.unwrap();
					let slot = data.create_slot(address).unwrap();
					mem::replace(&mut *slot.borrow_mut(), elem);
					redo.record(address, None);
					continue;
				},

				(false, true) => { // The slot was added.
					let cur = try!(data.remove_slot(address));
					redo.record(address, Some(cur));
					continue;
				},

				_ => panic!("null entry in Undo operation")
			}
		}

		Ok(HistoryEntry {
			info: EntryInfo::Undo(self.undoing.unwrap()),
			undo: Box::new(redo),
		})
	}
}

