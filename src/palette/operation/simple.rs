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
use super::{PaletteOperation, HistoryEntry, OperationInfo, set_target};
use super::undo::Undo;
use palette::{Result, Error};
use palette::data::PaletteOperationData;
use palette::element::ColorElement;
use address::Address;
use color::Color;



////////////////////////////////////////////////////////////////////////////////
// InsertColor
////////////////////////////////////////////////////////////////////////////////
/// Inserts a new color into the palette.
/// 
/// # Example
///
/// ```rust
/// use rampeditor::*;
/// 
/// let mut pal = Palette::new("Example", Format::Default, true);
///
/// pal.apply(Box::new(InsertColor::new(Color::new(12, 50, 78)))).unwrap();
///
/// assert_eq!(pal.get_color(Address::new(0, 0, 0)), Some(Color::new(12, 50, 78)));
/// 
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct InsertColor {
	/// The Color to add to the paletee.
	color: Color,
	/// The location to start placing the colors.
	location: Option<Address>,
	/// Whether to overwrite existing elements when generating new ones.
	overwrite: bool,
}


impl InsertColor {
	/// Creates a new InsertColor operation.
	#[inline]
	pub fn new<C>(color: C) -> InsertColor where C: Into<Color> {
		InsertColor {
			color: color.into(),
			location: None,
			overwrite: false,
		}
	}

	/// Sets the location to place the color.
	pub fn located_at(mut self, location: Address) -> InsertColor {
		self.location = Some(location);
		self
	}

	/// Configures the operation to overwrite existing elements when inserted.
	pub fn overwrite(mut self, overwrite: bool) -> InsertColor {
		self.overwrite = overwrite;
		self
	}
}


impl PaletteOperation for InsertColor {
	fn get_info(&self) -> OperationInfo {
		OperationInfo {
			name: "Insert Color",
			details: Some(format!("{:?}", self))
		}
	}

	fn apply(&mut self, data: &mut PaletteOperationData) -> Result<HistoryEntry> {
		// Get starting address.
		let starting_address = if let Some(address) = self.location {
			address
		} else {
			try!(data.first_free_address_after(Default::default()))
		};

		// Get targets.
		let target = try!(data.find_targets(
			1, 
			starting_address,
			self.overwrite,
			None
		))[0];

		// Check for derived color.
		if !self.overwrite && 
			data.get_slot(target).map_or(false, |slot| slot.get_order() != 0) 
		{
			return Err(Error::CannotSetDerivedColor);
		}

		// Create new color.
		let new_element = ColorElement::Pure {color: self.color};
		// Set target.
		let mut undo = Undo::new_for(self);
		try!(set_target(data, target, new_element, &mut undo));
		
		Ok(HistoryEntry {
			info: self.get_info(),
			undo: Box::new(undo),
		})
	}
}



////////////////////////////////////////////////////////////////////////////////
// RemoveElement
////////////////////////////////////////////////////////////////////////////////
/// Removes an element from the palette.
/// 
/// # Example
///
/// ```rust
/// use rampeditor::*;
/// 
/// let mut pal = Palette::new("Example", Format::Default, true);
///
/// pal.apply(Box::new(InsertColor::new(Color::new(12, 50, 78)))).unwrap();
/// pal.apply(Box::new(RemoveElement::new(Address::new(0, 0, 0)))).unwrap();
/// 
/// assert_eq!(pal.len(), 0);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct RemoveElement {
	/// The addres of the element to remove.
	address: Address,
}


impl RemoveElement {
	/// Creates a new RemoveElement operation targetting the given address.
	#[inline]
	pub fn new(address: Address) -> RemoveElement {
		RemoveElement {address: address}
	}
}


impl PaletteOperation for RemoveElement {
	fn get_info(&self) -> OperationInfo {
		OperationInfo {
			name: "Remove Element",
			details: Some(format!("{:?}", self))
		}
	}

	fn apply(&mut self, data: &mut PaletteOperationData) -> Result<HistoryEntry> {

		let mut undo = Undo::new_for(self);
		undo.record(self.address, Some(try!(data.remove_slot(self.address))));
		
		Ok(HistoryEntry {
			info: self.get_info(),
			undo: Box::new(undo),
		})
	}
}
