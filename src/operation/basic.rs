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
//! Defines simple color creation operations.
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use address::Address;
use data::Data;
use operation::{
	set_target,
	HistoryEntry,
	OperationInfo,
	PaletteOperation,
	Undo,
};
use result::Result;




////////////////////////////////////////////////////////////////////////////////
// InsertCell
////////////////////////////////////////////////////////////////////////////////
/// Inserts a new `Expression` into the palette.
/// 
/// # Example
///
/// ```rust
/// use palette::*;
/// 
/// let mut pal = Palette::new("Example", Format::Default, true);
///
/// pal.apply(Box::new(InsertCell::new())).unwrap();
///
/// assert_eq!(pal.color(Address::new(0, 0, 0)), None);
/// 
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct InsertCell {
	/// The location to start placing the colors.
	location: Option<Address>,
	/// Whether to overwrite existing cells when generating new ones.
	overwrite: bool,
}


impl InsertCell {
	/// Creates a new InsertCell operation.
	#[inline]
	pub fn new() -> InsertCell {
		InsertCell {
			location: None,
			overwrite: false,
		}
	}

	/// Sets the location to place the cell.
	pub fn located_at(mut self, location: Address) -> InsertCell {
		self.location = Some(location);
		self
	}

	/// Configures the operation to overwrite existing cells when inserted.
	pub fn overwrite(mut self, overwrite: bool) -> InsertCell {
		self.overwrite = overwrite;
		self
	}
}


impl PaletteOperation for InsertCell {
	fn info(&self) -> OperationInfo {
		OperationInfo {
			name: "Insert Cell",
			details: Some(format!("{:?}", self))
		}
	}


	fn apply(&mut self, data: &mut Data) -> Result<HistoryEntry> {
		// Get starting address.
		let starting_address = if let Some(address) = self.location {
			address
		} else {
			data.first_free_address_after(Default::default())?
		};

		// Get targets.
		let target = data.find_targets(
			1, 
			starting_address,
			self.overwrite,
			None
		)?[0];

		// Set target.
		let mut undo = Undo::new_for(self);
		set_target(data, target, Default::default(), &mut undo)?;
		
		Ok(HistoryEntry {
			info: self.info(),
			undo: Box::new(undo),
		})
	}
}



////////////////////////////////////////////////////////////////////////////////
// DeleteCell
////////////////////////////////////////////////////////////////////////////////
/// Removes an cell from the palette.
/// 
/// # Example
///
/// ```rust
/// use palette::*;
/// 
/// let mut pal = Palette::new("Example", Format::Default, true);
///
/// pal.apply(Box::new(InsertCell::new(Color::new(12, 50, 78)))).unwrap();
/// pal.apply(Box::new(DeleteCell::new(Address::new(0, 0, 0)))).unwrap();
/// 
/// assert_eq!(pal.len(), 0);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct DeleteCell {
	/// The addres of the cell to remove.
	address: Address,
}


impl DeleteCell {
	/// Creates a new DeleteCell operation targetting the given address.
	#[inline]
	pub fn new(address: Address) -> DeleteCell {
		DeleteCell {address: address}
	}
}


impl PaletteOperation for DeleteCell {
	fn info(&self) -> OperationInfo {
		OperationInfo {
			name: "Remove Cell",
			details: Some(format!("{:?}", self))
		}
	}

	fn apply(&mut self, data: &mut Data) -> Result<HistoryEntry> {

		let mut undo = Undo::new_for(self);
		undo.record(self.address, Some(data.remove_cell(self.address)?));
		
		Ok(HistoryEntry {
			info: self.info(),
			undo: Box::new(undo),
		})
	}
}