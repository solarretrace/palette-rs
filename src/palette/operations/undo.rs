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

use palette::Result;
use palette::data::PaletteData;
use palette::element::ColorElement;
use palette::history::{HistoryEntry, EntryInfo};
use palette::format::PaletteOperation;
use address::Address;

use std::mem;


////////////////////////////////////////////////////////////////////////////////
// Undo
////////////////////////////////////////////////////////////////////////////////
/// Restores a set of elements in the palette.
#[derive(Debug, Default)]
pub struct Undo {
	saved: Vec<(Address, Option<ColorElement>)>
}

impl Undo {
	/// Creates a new Undo operation.
	#[inline]
	pub fn new() -> Undo {Default::default()}

	/// Records an element change to be replayed by the Undo operation.
	#[inline]
	pub fn record(&mut self, address: Address, element: Option<ColorElement>) {
		self.saved.push((address, element));
	}
}


impl PaletteOperation for Undo {
	fn apply(self, data: &mut PaletteData) -> Result<HistoryEntry> {
		let mut redo = Undo::new();

		for (address, item) in self.saved.into_iter() {
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
					let cur = try!(data.remove_element(address));
					redo.record(address, Some(cur));
					continue;
				},

				_ => panic!("null entry in Undo operation")
			}
		}

		Ok(HistoryEntry {
			info: EntryInfo::Undo,
			undo: Box::new(redo),
		})
	}
}

