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
use color::{Color, lerp_rgb};
use address::Address;
use address;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::collections::btree_map::{Iter, Keys, Values};
use std::fmt;
use std::error;
use std::u8;
use std::mem;

/// Default page count for a new palette.
pub const DEFAULT_PAGE_COUNT: u8 = 32;
/// Default line count for a new palette.
pub const DEFAULT_LINE_COUNT: u8 = 16;
/// Default column count for a new palette.
pub const DEFAULT_COLUMN_COUNT: u8 = 16;


/// The upper limit on the number of colors that can be in a single palette.
pub const MAX_PALETTE_SIZE: usize = (
	u8::MAX as usize * u8::MAX as usize * u8::MAX as usize
);

////////////////////////////////////////////////////////////////////////////////
// Palette
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single palette.
#[derive(Debug)]
pub struct Palette {
	/// A map associating addresses to elements of the palette.
	address_map: BTreeMap<Address, PaletteElement>,
	/// The internal address cursor that is used to track the next available 
	/// address.
	address_cursor: address::Select,
	/// The number of pages in the palette.
	page_count: u8,
	/// The number of lines in each page.
	line_count: u8,
	/// The number of columns in each line.
	column_count: u8,
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

	/// Returns the Color associated with the given index, or None if there is 
	/// not element at the given address or the address is invalid.
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

	/// Sets the element at the given address to the given color. If the address
	/// is empty, a new element is created to hold it. If the target address 
	/// contains a derived color, or the given address lies outside the range 
	/// defined by the palette settings, an error will be returned. Otherwise, 
	/// the replaced color will be returned, or None if a new element was 
	/// created.
	pub fn set_color(
		&mut self, 
		address: Address, 
		color: Color) 
		-> Result<Option<Color>, Error>
	{
		if let Some(element) = self.address_map.get(&address) {
			if element.borrow().get_order() == 0 {
				let new = ColorElement::ZerothOrder {color: color};
				let old = mem::replace(&mut *element.borrow_mut(), new);
				return Ok(Some(old.get_color()));
			} 
			return Err(Error::CannotSetDerivedColor);
		} 

		let element = Rc::new(RefCell::new(ColorElement::ZerothOrder {
			color: color
		}));
		self.address_map.insert(address, element);
		Ok(None)
	}

	/// Adds to the Palette a linearly interpolated RGB color ramp of the given 
	/// length between the colors given by their indices in the palette. Returns
	/// the end address if the length is 0 or if the start and and addresses are
	/// the same. Returns an error if there is not enough space for the ramp or
	/// if an invalid address is given.
	pub fn add_ramp_between(
		&mut self, 
		start_address: Address, 
		end_address: Address, 
		length: u8) 
		-> Result<Address, Error>
	{	
		// Check if there's enough space.
		if self.space_remaining() < length as usize {
			return Err(self.get_overflow_error());
		}

		// Error if invalid addresses given.
		if !self.valid_address(start_address) || 
			!self.valid_address(end_address) 
		{
			return Err(Error::InvalidAddress);
		}

		// Return if ramp would have no colors.
		if length == 0 || start_address == end_address {
			return Ok(end_address);
		}

		let mut address = start_address;
		for i in 0..length {
			let p1 = try!(self.address_map
				.get(&start_address)
				.ok_or(Error::EmptyAddress(start_address))).clone();
			let p2 = try!(self.address_map
				.get(&end_address)
				.ok_or(Error::EmptyAddress(end_address))).clone();
			
			// Compute distance between points for this element.
			let factor = (i + 1) as f32 * (1.0 / (length + 1) as f32);

			address = self.next_free_address_advance_cursor()
				.expect("computing addresses for ramp");

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


	/// Returns the next available address after the cursor. Returns an error if
	/// there are no free addresses.
	fn next_free_address(&self) -> Result<Address, Error> {
		if self.space_remaining() == 0 {
			return Err(self.get_overflow_error());
		}

		let mut address = self.address_cursor.base_address();
		while self.address_map.contains_key(&address) {
			address = address.wrapped_next(
				self.page_count,
				self.line_count, 
				self.column_count
			);
		}
		Ok(address)
	}

	/// Returns the next available address after the cursor, and also advances
	/// the cursor to the next (wrapped) address. Returns an error and fails to 
	/// advance the cursor if there are no free addresses.
	fn next_free_address_advance_cursor(&mut self) -> Result<Address, Error> {
		let address = try!(self.next_free_address());
		// Update the cursor.
		self.address_cursor = address.wrapped_next(
			self.page_count,
			self.line_count, 
			self.column_count
		).into();
		Ok(address)
	}

	/// Returns whether the give address lies within the bounds defined by the 
	/// wrapping and max page settings for the palette.
	fn valid_address(&self, address: Address) -> bool {
		address.page <= self.page_count &&
		address.line < self.line_count &&
		address.column < self.column_count
	}

	/// Returns the upper bound on the number of elements storable in the 
	/// palette.
	fn size_bound(&self) -> usize {
		self.page_count as usize * 
		self.line_count as usize * 
		self.column_count as usize
	}

	/// Returns the amount of room left in the palette.
	fn space_remaining(&self) -> usize {
		self.size_bound() - self.address_map.len()
	}

	/// Returns whether there are addresses that the palette considers invalid.
	fn overflow_possible(&self) -> bool {
		self.column_count < u8::MAX ||
		self.line_count < u8::MAX ||
		self.page_count < u8::MAX
	}

	/// Returns the approprate error for an overflow condition.
	fn get_overflow_error(&self) -> Error {
		if self.overflow_possible() {
			return Error::SetElementLimitExceeded;
		} else {
			return Error::MaxElementLimitExceeded;
		}
	}
}


impl fmt::Display for Palette {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		try!(write!(f, "Palette [{} pages] [wrap {}:{}] [address::select {}] \
						[{} slots free]\n\
			            \tAddress   Color    Order\n",
			self.page_count,
			self.line_count,
			self.column_count,
			self.address_cursor,
			self.space_remaining()

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
			page_count: DEFAULT_PAGE_COUNT,
			line_count: DEFAULT_LINE_COUNT,
			column_count: DEFAULT_COLUMN_COUNT,
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
	/// Attempted to add a color to the palette, but the current wrapping 
	/// settings prevent adding the color within the defined ranges. (Overflow
	/// is possible.)	
	SetElementLimitExceeded,
	/// Attempted to add a color to the palette, but the palette contains the 
	/// maximum number of elements already. (Overflow not possible.)
	MaxElementLimitExceeded,
	/// Attempted to set a color to a non-zeroth-order element.
	CannotSetDerivedColor,
	/// An address was provided that lies outside of the range defined for the 
	/// palette.
	InvalidAddress,
	/// An empty address was provided for an operation that requires a color.
	EmptyAddress(Address)
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match *self {
			Error::EmptyAddress(address) => write!(f, "{}: {}", 
				error::Error::description(self), 
				address
			),

			_ => write!(f, "{}", error::Error::description(self))
		}
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
			Error::CannotSetDerivedColor
				=> "cannot assign color to a location containing a derived \
				    color value",
			Error::InvalidAddress
				=> "address provided is outside allowed range for palette",
			Error::EmptyAddress(..)
				=> "empty address provided to an operation requiring a color"
		}
	}
}



////////////////////////////////////////////////////////////////////////////////
// PaletteBuilder
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates the state of the palette during builder pattern construction.
pub struct PaletteBuilder {
	/// The internal address cursor that is used to track the next available 
	/// address.
	address_cursor: address::Select,
	/// The number of pages in the palette.
	page_count: u8,
	/// The number of lines in each page.
	line_count: u8,
	/// The number of columns in each line.
	column_count: u8,
}


impl PaletteBuilder {
	/// Starts building the palette with the default settings.
	pub fn new() -> PaletteBuilder {
		Default::default()
	}

	/// Sets the max page count.
	pub fn with_page_count(mut self, page_count: u8) -> PaletteBuilder {
		self.page_count = page_count;
		self
	}

	/// Sets the line wrap for new elements.
	pub fn with_line_count(mut self, line_count: u8) -> PaletteBuilder {
		self.line_count = line_count;
		self
	}
	
	/// Sets the max page count.
	pub fn with_column_count(mut self, column_count: u8) -> PaletteBuilder {
		self.column_count = column_count;
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
			page_count: self.page_count,
			line_count: self.line_count,
			column_count: self.column_count,
			address_map: BTreeMap::new(),
		}
	}
}

impl Default for PaletteBuilder {
	fn default() -> Self {
		PaletteBuilder {
			address_cursor: address::Select::All,
			page_count: DEFAULT_PAGE_COUNT,
			line_count: DEFAULT_LINE_COUNT,
			column_count: DEFAULT_COLUMN_COUNT,
		}
	}
}