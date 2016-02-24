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
//! Defines wrapper structs for common palette operations.
//!
////////////////////////////////////////////////////////////////////////////////
use super::data::*;
use super::error::{Result, Error};
use super::element::{Slot, ColorElement};
use address::Address;

use std::rc::Rc;

fn retrieve_source(
	palette: &mut PaletteData, 
	address: Address, 
	make_sources: bool) 
	-> Result<Rc<Slot>> 
{
	if make_sources {
		palette.get_or_create_slot(address)
			.map(|slot| slot
				.upgrade()
				.expect("upgrade valid slot reference"))
	} else {
		palette
			.get_slot(address)
			.map(|slot| slot
				.upgrade()
				.expect("upgrade valid slot reference"))
			.ok_or_else(|| if palette.check_address(address) {
					Error::EmptyAddress(address)
				} else {
					Error::InvalidAddress(address)
				})
	}
}


////////////////////////////////////////////////////////////////////////////////
// PaletteOperation
////////////////////////////////////////////////////////////////////////////////
/// Provides the methods for modifying palettes.
pub trait PaletteOperation {
	/// Applies the operation to the given palette.
	fn apply(&self, palette: &mut PaletteData) -> Result<()>;
}



////////////////////////////////////////////////////////////////////////////////
// CreateRamp
////////////////////////////////////////////////////////////////////////////////
/// Creates a linear RGB color ramp using second-order elements in the palette.
pub struct CreateRamp {
	// The location to start placing the ramp elements.
	location: Option<Address>,
	// The address of the starting color of the ramp.
	from: Address,
	// The address of the ending color of the ramp.
	to: Address,
	// The number of elements to create.
	count: usize,
	// Whether to overwrite existing elements when generating new ones.
	overwrite: bool,
	// Whether to generate placeholder slots when an invalid reference is given.
	make_sources: bool,
}


impl CreateRamp {

	/// Creates a new CreateRamp operation.
	#[inline]
	pub fn new(from: Address, to: Address, count: usize) -> CreateRamp {
		CreateRamp {
			location: None,
			from: from,
			to: to,
			count: count,
			overwrite: false,
			make_sources: false,
		}
	}

	/// Sets the location to start placing elements for the operation.
	pub fn located_at(mut self, location: Address) -> CreateRamp {
		self.location = Some(location);
		self
	}

	/// Configures the operation to overwrite existing elements as it generates
	/// new elements. This will ensure that the generated ramp is contiguous in
	/// the palette, but will produce an error if it would overwrite a 
	/// dependency.
	pub fn overwrite(mut self, overwrite: bool) -> CreateRamp {
		self.overwrite = overwrite;
		self
	}

	/// Configures the operation to generate placeholder colors instead of 
	/// producing an error when empty addresses are provided. 
	pub fn make_sources(
		mut self, 
		make_sources: bool) 
		-> CreateRamp 
	{
		self.make_sources = make_sources;
		self
	}
}


impl PaletteOperation for CreateRamp {
	fn apply(&self, palette: &mut PaletteData) -> Result<()> {
		
		// Validate source addresses.
		if !palette.check_address(self.from) {
			return Err(Error::InvalidAddress(self.from));
		}
		if !palette.check_address(self.to) {
			return Err(Error::InvalidAddress(self.to));
		}

		// Retrieve or create sources.
		let from_slot = try!(retrieve_source(
			palette, 
			self.from, 
			self.make_sources
		));
		let to_slot = try!(retrieve_source(
			palette, 
			self.to, 
			self.make_sources
		));


		// Get and verify sources for mix function.
		// if palette.check_address(self.from) {

		// } else {

		

		// // Generate new elements.
		// let mut new_elements = Vec::new();
		// for i in 0..self.count {
		// 	new_elements.push(ColorElement::Mixed {
		// 		mix: Box::new(|sources| {
		// 			Default::default()
		// 		}),
		// 		sources: [
		// 			palette.slotmap.get(&self.from).downgrade(),
		// 			palette.slotmap.get(&self.to).downgrade()
		// 		].into_iter().collect()
		// 	})
		// }

		// Add new elements to palette.

		Ok(())
	}
}