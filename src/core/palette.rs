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

//! Defines a structured Palette object for storing and generating colors.
use super::element::{ColorElement, PaletteElement};
use super::color::{Color, lerp_rgb};
use super::address::Address;
use super::address;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::collections::btree_map::{Iter, Keys, Values};
use std::fmt;
use std::error;
use std::u8;

const DEFAULT_MAX_PAGE_COUNT: u8 = 32;
const DEFAULT_COLUMN_WRAP: u8 = 16;
const DEFAULT_LINE_WRAP: u8 = 16;

const MAX_PALETTE_ELEMENT_COUNT: usize = (
	u8::MAX as usize * u8::MAX as usize * u8::MAX as usize
);

////////////////////////////////////////////////////////////////////////////////
// Palette
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single palette.
#[derive(Debug)]
pub struct Palette {
	// A map associating addresses to elements of the palette.
	address_map: BTreeMap<Address, PaletteElement>,
	// The internal address cursor that is used to track the next available 
	// address.
	address_cursor: address::Select,
	// The maximum page count allowed by the palette.
	max_page_count: u8,
	// The line and column wrapping to use when generating new addresses.
	line_wrap: u8,
	column_wrap: u8,
}


impl Palette {
	
	/// Constructs a new, empty Palette.
	pub fn new() -> Palette {
		Default::default()
	}


	/// Returns the number of colors in the Palette.
	pub fn len(&self) -> usize {
		self.address_map.len()
	}

	/// Returns the Color associated with the given index.
	pub fn get_color(&self, address: Address) -> Option<Color> {
		self.address_map
			.get(&address)
		    .map(|ref element| element.borrow().get_color())
	}

	/// Adds the given color to the palette and returns the address of its 
	/// location.
	pub fn add_color(&mut self, color: Color) -> Result<Address, Error> {
		let address = try!(self.next_free_address_advance_cursor());
		let element = Rc::new(RefCell::new(ColorElement::ZerothOrder {
			color: color
		}));
		self.address_map.insert(address, element);
		Ok(address)
	}

	/// Adds to the Palette a linearly interpolated RGB color ramp of the given 
	/// length between the colors given by their indices in the palette.
	pub fn add_ramp_between(
		&mut self, 
		start_address: Address, 
		end_address: Address, 
		length: u8) 
		-> Result<Address, Error>
	{
		// Return if ramp would have no colors.
		if length == 0 || start_address == end_address {
			return Ok(end_address);
		}
		let mut address = start_address;
		for i in 0..length {
			let p1 = self.address_map.get(&start_address).unwrap().clone();
			let p2 = self.address_map.get(&end_address).unwrap().clone();
			
			// Compute distance between points for this element.
			let factor = (i + 1) as f32 * (1.0 / (length + 1) as f32);

			address = try!(self.next_free_address_advance_cursor());
			let element = Rc::new(RefCell::new(ColorElement::SecondOrder{
				build: Box::new(move |a, b| {
					// Build color by doing lerp with computed factor.
					lerp_rgb(a.get_color(), b.get_color(), factor)
				}),
				parents: (p1, p2)
			}));
			self.address_map.insert(address, element);
		}
		Ok(address)
	}

	/// Returns an iterator over the (Address, Color) entries of the palette.
	pub fn iter(&self) -> PaletteIterator {
		PaletteIterator::new(self)
	}

	/// Returns and iterator over the colors of the palette in address order.
	pub fn colors(&self) -> ColorIterator {
		ColorIterator::new(self)
	}

	/// Returns and iterator over the addresses of the palette in order.
	pub fn addresses(&self) -> AddressIterator {
		AddressIterator::new(self)
	}


	/// Returns the next available address after the cursor.
	fn next_free_address(&self) -> Result<Address, Error> {
		let mut address = self.address_cursor.base_address();
		while self.address_map.contains_key(&address) {
			let res = address.wrapped_next(
				self.line_wrap, 
				self.column_wrap
			);
			// Check for addressing errors.
			if res.is_err() {
				if self.address_map.len() < MAX_PALETTE_ELEMENT_COUNT {
					return Err(Error::SetElementLimitExceeded);
				} else {
					return Err(Error::MaxElementLimitExceeded);
				}
			}
			address = res.ok().unwrap();
			// Check for page limit errors.
			if address.page > self.max_page_count {
				return Err(Error::SetElementLimitExceeded)
			}
		}
		Ok(address)
	}

	/// Returns the next available address after the cursor, and also advances
	/// the cursor to the next (wrapped) address.
	fn next_free_address_advance_cursor(&mut self) -> Result<Address, Error> {
		let address = try!(self.next_free_address());
		
		// Update the cursor.
		self.address_cursor = address.wrapped_next(
			self.line_wrap, 
			self.column_wrap
		).unwrap_or(Default::default()).into();
		Ok(address)
	}
}


impl fmt::Display for Palette {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		try!(write!(f, "Palette [wrap {}:{}] [address::select {}]\n\
			            \tAddress   Color    Order\n",
			self.line_wrap,
			self.column_wrap,
			self.address_cursor
		));

		for (&address, ref element) in self.address_map.iter() {
			try!(write!(f, "\t{:X}  {:X}  {}\n",
				address,
				element.borrow().get_color(),
				match &*element.borrow() {
					&ColorElement::ZerothOrder {..} => "0",
					&ColorElement::FirstOrder {..} => "1",
					&ColorElement::SecondOrder {..} => "2"
				}
			));
		}
		Ok(())
	}
}

impl Default for Palette {
	fn default() -> Self {
		Palette {
			address_cursor: address::Select::All,
			max_page_count: DEFAULT_MAX_PAGE_COUNT,
			line_wrap: DEFAULT_LINE_WRAP,
			column_wrap: DEFAULT_COLUMN_WRAP,
			address_map: BTreeMap::new(),
		}
	}
}

////////////////////////////////////////////////////////////////////////////////
// PaletteIterator
////////////////////////////////////////////////////////////////////////////////
/// An iterator over the (Address, Color) entries of a palette. The entries are 
/// returned in address order.
pub struct PaletteIterator<'p> {
	inner: Iter<'p, Address, PaletteElement>
}


impl<'p> PaletteIterator<'p> {
	fn new(palette: &'p Palette) -> Self {
		PaletteIterator {inner: palette.address_map.iter()}
	}
}


impl<'p> Iterator for PaletteIterator<'p> {
	type Item = (Address, Color);

	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next().map(|(&address, ref element)| 
			(address, element.borrow().get_color())
		)
	}
}


////////////////////////////////////////////////////////////////////////////////
// ColorIterator
////////////////////////////////////////////////////////////////////////////////
/// An iterator over the colors of a palette. The colors are returned in address 
/// order.
pub struct ColorIterator<'p> {
	inner: Values<'p, Address, PaletteElement>
}


impl<'p> ColorIterator<'p> {
	fn new(palette: &'p Palette) -> Self {
		ColorIterator {inner: palette.address_map.values()}
	}
}


impl<'p> Iterator for ColorIterator<'p> {
	type Item = Color;

	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next().map(|ref element| element.borrow().get_color())
	}
}


////////////////////////////////////////////////////////////////////////////////
// AddressIterator
////////////////////////////////////////////////////////////////////////////////
/// An iterator over the colors of a palette. The colors are returned in address 
/// order.
pub struct AddressIterator<'p> {
	inner: Keys<'p, Address, PaletteElement>
}


impl<'p> AddressIterator<'p> {
	fn new(palette: &'p Palette) -> Self {
		AddressIterator {inner: palette.address_map.keys()}
	}
}


impl<'p> Iterator for AddressIterator<'p> {
	type Item = Address;

	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next().map(|&address| address)
	}
}


////////////////////////////////////////////////////////////////////////////////
// Error
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates errors associated with mutating palette operations.
#[derive(Debug)]
pub enum Error {
	/// User attempted to add a color to the palette, but the current wrapping 
	/// settings prevent adding the color within the defined ranges. (Overflow
	/// is possible.)	
	SetElementLimitExceeded,
	/// User attempted to add a color to the palette, but the palette contains
	/// the maximum number of elements already. (Overflow not possible.)
	MaxElementLimitExceeded,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", error::Error::description(self))
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::SetElementLimitExceeded 
				=> "maximum number of color elements for wrapping settings \
					exceeded",

			Error::MaxElementLimitExceeded
				=> "maximum number of color elements for palette exceeded",
		}
	}
	// fn cause(&self) -> Option<&Error> {None}
}


////////////////////////////////////////////////////////////////////////////////
// PaletteBuilder
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates the state of the palette during builder pattern construction.
pub struct PaletteBuilder {
	// The internal address cursor that is used to track the next available 
	// address.
	address_cursor: address::Select,
	// The maximum page count allowed by the palette.
	max_page_count: u8,
	// The line and column wrapping to use when generating new addresses.
	line_wrap: u8,
	column_wrap: u8,
}


impl PaletteBuilder {
	/// Starts building the palette with the default settings.
	pub fn new() -> PaletteBuilder {
		Default::default()
	}

	/// Sets the max page count.
	pub fn with_max_page_count(mut self, max_page_count: u8) -> PaletteBuilder {
		self.max_page_count = max_page_count;
		self
	}

	/// Sets the line wrap for new elements.
	pub fn with_line_wrap(mut self, line_wrap: u8) -> PaletteBuilder {
		self.line_wrap = line_wrap;
		self
	}
	
	/// Sets the max page count.
	pub fn with_column_wrap(mut self, column_wrap: u8) -> PaletteBuilder {
		self.column_wrap = column_wrap;
		self
	}

	/// Sets the starting address cursor.
	pub fn with_starting_address_cursor(
		mut self, 
		address_cursor: address::Select) 
		-> PaletteBuilder
	{
		self.address_cursor = address_cursor;
		self
	}
	

	/// Builds the palette and returns it.
	pub fn build(self) -> Palette {
		Palette {
			address_cursor: self.address_cursor,
			max_page_count: self.max_page_count,
			line_wrap: self.line_wrap,
			column_wrap: self.column_wrap,
			address_map: BTreeMap::new(),
		}
	}
}

impl Default for PaletteBuilder {
	fn default() -> Self {
		PaletteBuilder {
			address_cursor: address::Select::All,
			max_page_count: DEFAULT_MAX_PAGE_COUNT,
			line_wrap: DEFAULT_LINE_WRAP,
			column_wrap: DEFAULT_COLUMN_WRAP,
		}
	}
}