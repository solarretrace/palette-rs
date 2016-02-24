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
use super::error::Result;
// use color::Color;
use address::Address;
// use address;

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
	generate_placeholders: bool,
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
			generate_placeholders: false,
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
	pub fn generate_placeholders(
		mut self, 
		generate_placeholders: bool) 
		-> CreateRamp 
	{
		self.generate_placeholders = generate_placeholders;
		self
	}
}


impl PaletteOperation for CreateRamp {
	fn apply(&self, palette: &mut PaletteData) -> Result<()> {
		

		Ok(())
	}
}