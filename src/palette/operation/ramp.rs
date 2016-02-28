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
//! Defines ramp creation operations.
//!
////////////////////////////////////////////////////////////////////////////////
use super::common::{PaletteOperation, get_source, set_target};
use palette::Result;
use palette::data::PaletteData;
use palette::element::ColorElement;
use palette::history::{HistoryEntry, EntryInfo};
use palette::operation::Undo;
use address::Address;
use color::lerp_rgb;




////////////////////////////////////////////////////////////////////////////////
// CreateRamp
////////////////////////////////////////////////////////////////////////////////
/// Creates a linear RGB color ramp using second-order elements in the palette.
#[derive(Debug, Clone, Copy, Default)]
pub struct CreateRamp {
	/// The location to start placing the ramp elements.
	location: Option<Address>,
	/// The address of the starting color of the ramp.
	from: Address,
	/// The address of the ending color of the ramp.
	to: Address,
	/// The number of elements to create.
	count: usize,
	/// Whether to overwrite existing elements when generating new ones.
	overwrite: bool,
	/// Whether to generate placeholder slots when an invalid reference is given.
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
	/// new elements.
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
	fn apply(self, data: &mut PaletteData) -> Result<HistoryEntry> {
		
		// Get starting address.
		let starting_address = if let Some(address) = self.location {
			address
		} else {
			try!(data.first_free_address_after(Default::default()))
		};

		// Get target addresses.
		let targets = try!(data.find_targets(
			self.count, 
			starting_address,
			self.overwrite,
			Some(vec![self.from, self.to]) // Exclude the source locations.
		));

		// Get source slots.
		let mut undo = Undo::new_for(&self);
		let make = self.make_sources;
		let src_from = try!(get_source(data, self.from, make, &mut undo));
		let src_to = try!(get_source(data, self.to, make, &mut undo));
				
		// Generate ramp.
		for (i, &address) in targets.iter().enumerate() {
			let am = (1.0 / (self.count + 2) as f32) * (i + 1) as f32;

			let new_element = ColorElement::Mixed {
				mix: Box::new(move |colors| lerp_rgb(colors[0], colors[1], am)),
				sources: vec![src_from.clone(), src_to.clone()]
			};

			try!(set_target(data, address, new_element, &mut undo));
		}

		Ok(HistoryEntry {
			info: EntryInfo::Apply(Box::new(self)),
			undo: Box::new(undo)
		})
	}
}

