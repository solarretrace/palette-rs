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

use super::common::{PaletteOperation, set_target};
use palette::{Result, Error};
use palette::data::PaletteData;
use palette::element::ColorElement;
use palette::history::{HistoryEntry, EntryInfo};
use palette::operations::Undo;
use address::Address;
use color::Color;




////////////////////////////////////////////////////////////////////////////////
// CreateColor
////////////////////////////////////////////////////////////////////////////////
/// Creates a new color in the palette.
#[derive(Debug, Clone, Copy, Default)]
pub struct CreateColor {
	/// The Color to add to the paletee.
	color: Color,
	/// The location to start placing the colors.
	location: Option<Address>,
	/// Whether to overwrite existing elements when generating new ones.
	overwrite: bool,
}


impl CreateColor {

	/// Creates a new CreateColor operation.
	#[inline]
	pub fn new(color: Color) -> CreateColor {
		CreateColor {
			color: color,
			location: None,
			overwrite: false,
		}
	}

	/// Sets the location to start placing elements for the operation.
	pub fn located_at(mut self, location: Address) -> CreateColor {
		self.location = Some(location);
		self
	}

	/// Configures the operation to overwrite existing elements as it generates
	/// new elements. This will ensure that the generated ramp is contiguous in
	/// the palette, but will produce an error if it would overwrite a 
	/// dependency.
	pub fn overwrite(mut self, overwrite: bool) -> CreateColor {
		self.overwrite = overwrite;
		self
	}
}


impl PaletteOperation for CreateColor {
	fn apply(self, data: &mut PaletteData) -> Result<HistoryEntry> {
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
			data.get_slot(target).map_or(true, |slot| slot.get_order() != 1) 
		{
			return Err(Error::CannotSetDerivedColor);
		}

		

		// Create new color.
		let new_element = ColorElement::Pure {color: self.color};

		// Set target.
		let mut undo = Undo::new();
		try!(set_target(data, target, new_element, &mut undo));
		

		Ok(HistoryEntry {
			info: EntryInfo::Apply {operation: Box::new(self)},
			undo: Box::new(undo),
		})
	}
}