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
use super::element::Slot;
use super::metadata::Metadata;
use super::operations::PaletteOperation;
use super::format::{PaletteFormat};
use color::Color;
use address::Address;
use address;

use std::rc::Rc;
use std::collections::BTreeMap;
use std::collections::btree_map::{Iter, Keys};
use std::fmt;
use std::error;
use std::u8;


////////////////////////////////////////////////////////////////////////////////
// Palette
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single palette.
#[derive(Debug)]
pub struct Palette {
	/// The PaletteFormat used to configure the palette.
	format_type: Option<&'static PaletteFormat>,
	/// The version of the PaletteFormat used to configure the palette.
	format_version: Option<(u8, u8, u8)>,
	/// A map assigning addresses to palette slots.
	data: BTreeMap<Address, Rc<Slot>>,
	/// Provided metadata for various parts of the palette.
	metadata: BTreeMap<address::Select, Metadata>,
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
	#[inline]
	pub fn new() -> Palette {
		Default::default()
	}

	/// Returns the number of colors in the Palette.
	#[inline]
	pub fn len(&self) -> usize {
		self.data.len()
	}

	/// Returns the palette's current selection cursor.
	pub fn get_cursor(&self) -> address::Select {
		self.address_cursor
	}

	/// Sets the palette's selection cursor.
	pub fn set_cursor(&mut self, new_selection: address::Select) {
		if self.check_address(new_selection.base_address()) {
			self.address_cursor = new_selection;
		}
	}

	pub fn execute(&mut self, op: &PaletteOperation) -> Result<(), Error> {
		Ok(())
	}

	/// Adds a new slot to the palette in the nearest valid location after the 
	/// selection cursor and returns its address. Returns an error if the 
	/// palette is full.
	pub fn add_slot(&mut self, slot: Slot) -> Result<Address, Error> {
		let address = try!(self.next_free_address_advance_cursor());
		self.data.insert(address, Rc::new(slot));
		Ok(address)
	}

	/// Returns the slot located at the given address, or `None` if the address
	/// is invalid or empty.
	pub fn get_slot(&self, address: Address) -> Option<&Rc<Slot>> {
		self.data.get(&address)
	}

	/// Assigns the given slot to the given address. Returns the slot previously
	/// located at that position, or `None` if it was empty. Returns an error if
	/// the address is invalid.
	pub fn set_slot(
		&mut self, 
		address: Address, 
		slot: Slot) 
		-> Result<Option<Rc<Slot>>, Error> 
	{
		if self.check_address(address) {
			Ok(self.data
				.insert(address, Rc::new(slot))
			)
		} else {
			Err(Error::InvalidAddress)
		}
	}
	
	/// Returns an iterator over the (Address, Color) entries of the palette.
	#[inline]
	pub fn iter(&self) -> PaletteIterator {
		PaletteIterator::new(self)
	}

	/// Returns an iterator over the colors of the palette in address order.
	#[inline]
	pub fn colors(&self) -> ColorIterator {
		ColorIterator::new(self)
	}

	/// Returns and iterator over the addresses of the palette in order.
	#[inline]
	pub fn addresses(&self) -> AddressIterator {
		AddressIterator::new(self)
	}

	/// Returns the next available address after the cursor, and also advances
	/// the cursor to the next (wrapped) address. Returns an error and fails to 
	/// advance the cursor if there are no free addresses.
	#[inline]
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

	/// Returns the next available address after the cursor. Returns an error if
	/// there are no free addresses.
	#[inline]
	fn next_free_address(&self) -> Result<Address, Error> {
		if self.space_remaining() == 0 {
			return Err(Error::MaxSlotLimitExceeded);
		}

		let mut address = self.address_cursor.base_address();
		while self.data.get(&address).and_then(|s| s.get_color()).is_some() {
			address = address.wrapped_next(
				self.page_count,
				self.line_count, 
				self.column_count
			);
		}
		Ok(address)
	}

	/// Returns whether the give address lies within the bounds defined by the 
	/// wrapping and max page settings for the palette.
	#[inline]
	fn check_address(&self, address: Address) -> bool {
		address.page <= self.page_count &&
		address.line < self.line_count &&
		address.column < self.column_count
	}

	/// Returns the upper bound on the number of slots storable in the palette.
	#[inline]
	fn size_bound(&self) -> usize {
		self.page_count as usize * 
		self.line_count as usize * 
		self.column_count as usize
	}

	/// Returns the amount of room left in the palette.
	#[inline]
	fn space_remaining(&self) -> usize {
		self.size_bound() - self.data.len()
	}
}


impl fmt::Display for Palette {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		try!(write!(f, "Palette"));
		if let Some(data) = self.metadata.get(&address::Select::All) {
			try!(write!(f, " {}\n", data));
		}
		if let Some(format) = self.format_type {
			let version = format.get_version();
			try!(write!(f, 
				"[{} {}.{}.{}]", 
				format.get_name(),
				version.0,
				version.1,
				version.2));
		}
		try!(write!(f, 
			" [{} pages] [wrap {}:{}] [cursor {}] \
			[{} slots free]",
			self.page_count,
			self.line_count,
			self.column_count,
			self.address_cursor,
			self.space_remaining()
		));
		

		try!(write!(f, "\n\tAddress   Color    Order  Name\n"));
		for (&address, ref slot) in self.data.iter() {
			try!(write!(f, "\t{:X}  {:X}  {:<5}  ",
				address,
				slot.borrow().get_color().unwrap_or(Color(0,0,0)),
				slot.borrow().get_order()
			));
			if let Some(data) = self.metadata.get(&address.clone().into()) {
				try!(write!(f, "{}\n", data));
			} else {
				try!(write!(f, "-\n"));
			}
		}
		Ok(())
	}
}


impl Default for Palette {
	fn default() -> Self {
		Palette {
			format_type: None,
			format_version: None,
			data: BTreeMap::new(),
			metadata: BTreeMap::new(),
			address_cursor: address::Select::All,
			page_count: u8::MAX,
			line_count: u8::MAX,
			column_count: u8::MAX,
		}
	}
}



////////////////////////////////////////////////////////////////////////////////
// PaletteIterator
////////////////////////////////////////////////////////////////////////////////
/// An iterator over the (Address, Color) entries of a palette. The entries are 
/// returned in address order.
pub struct PaletteIterator<'p> {
	inner: Iter<'p, Address, Rc<Slot>>
}


impl<'p> PaletteIterator<'p> {
	fn new(palette: &'p Palette) -> Self {
		PaletteIterator {
			inner: palette.data.iter()
		}
	}
}


impl<'p> Iterator for PaletteIterator<'p> {
	type Item = (Address, Color);

	fn next(&mut self) -> Option<Self::Item> {
		if let Some((&address, ref slot)) = self.inner.next() {
			Some((address, 
				slot.get_color()
					.expect("iterator unwrapped valid slot")))
		} else {
			None
		}
	}
}


////////////////////////////////////////////////////////////////////////////////
// ColorIterator
////////////////////////////////////////////////////////////////////////////////
/// An iterator over the colors of a palette. The colors are returned in address 
/// order.
pub struct ColorIterator<'p> {
	inner: PaletteIterator<'p>
}


impl<'p> ColorIterator<'p> {
	fn new(palette: &'p Palette) -> Self {
		ColorIterator {inner: palette.iter()}
	}
}


impl<'p> Iterator for ColorIterator<'p> {
	type Item = Color;

	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next().map(|item| item.1)
	}
}


////////////////////////////////////////////////////////////////////////////////
// AddressIterator
////////////////////////////////////////////////////////////////////////////////
/// An iterator over the colors of a palette. The colors are returned in address 
/// order.
pub struct AddressIterator<'p> {
	inner: Keys<'p, Address, Rc<Slot>>
}


impl<'p> AddressIterator<'p> {
	fn new(palette: &'p Palette) -> Self {
		AddressIterator {inner: palette.data.keys()}
	}
}


impl<'p> Iterator for AddressIterator<'p> {
	type Item = Address;

	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next().map(|&address| address)
	}
}



////////////////////////////////////////////////////////////////////////////////
// PaletteBuilder
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates the state of the palette during builder pattern construction.
pub struct PaletteBuilder {
	/// The PaletteFormat used to configure the palette.
	format_type: Option<&'static PaletteFormat>,
	/// The version of the PaletteFormat used to configure the palette.
	format_version: Option<(u8, u8, u8)>,
	/// The internal address cursor that is used to track the next available 
	/// address.
	address_cursor: address::Select,
	/// The number of pages in the palette.
	page_count: u8,
	/// The number of lines in each page.
	line_count: u8,
	/// The number of columns in each line.
	column_count: u8,
	/// The name of the palette.
	palette_name: Option<String>,
}


impl PaletteBuilder {

	/// Starts building the palette with the default settings.
	pub fn new() -> PaletteBuilder {
		Default::default()
	}


	/// Allows the given palette format specification to set the palette's 
	/// properties.
	pub fn using_format<T>(mut self, format: &'static T) -> PaletteBuilder 
		where T: PaletteFormat 
	{
		self.format_type = Some(format);
		self.format_version = Some(format.get_version());
		format.configure(self)
	}


	/// Sets the palette name.
	pub fn named<S>(mut self, palette_name: S) -> PaletteBuilder 
		where S: Into<String>
	{
		self.palette_name = Some(palette_name.into());
		self
	}


	/// Sets the max page count.
	pub fn with_page_count(mut self, page_count: u8) -> PaletteBuilder {
		self.page_count = page_count;
		self
	}


	/// Sets the line wrap for new slots.
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
	pub fn create(self) -> Palette {
		let mut pal = Palette {
			format_type: self.format_type,
			format_version: self.format_version,
			address_cursor: self.address_cursor,
			page_count: self.page_count,
			line_count: self.line_count,
			column_count: self.column_count,
			.. Default::default()
		};

		if let Some(name) = self.palette_name {
			pal.metadata.insert(address::Select::All, Metadata::Name(name));
		}
		pal
	}
}


impl Default for PaletteBuilder {
	fn default() -> Self {
		PaletteBuilder {
			format_type: None,
			format_version: None,
			address_cursor: address::Select::All,
			page_count: u8::MAX,
			line_count: u8::MAX,
			column_count: u8::MAX,
			palette_name: None,
		}
	}
}



////////////////////////////////////////////////////////////////////////////////
// Error
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates errors associated with mutating palette operations.
#[derive(Debug)]
pub enum Error {
	/// Attempted to add a color to the palette, but the palette contains the 
	/// maximum number of slots already.
	MaxSlotLimitExceeded,
	/// Attempted to set a color to a non-zeroth-order slot.
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
			Error::MaxSlotLimitExceeded
				=> "maximum number of color slots for palette exceeded",
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