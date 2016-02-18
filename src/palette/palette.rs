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
//! Defines a structured Palette object for storing and generating colors.
//!
//! The palette acts as a tree-like structure that acts as a collection of 
//! 'Slots' into which color elements are placed. Color elements will then 
//! lazily generate a color when queried. This allows for the construction of 
//! dynamic palette structures that can generate related colors based off of a 
//! small subset of 'control' colors.
//!
//! More practically, `Slot`s are identified by `Address`, and each slot 
//! contains a single `ColorElement`, which will generate a `Color` when either
//! the Slot's or ColorElement's `get_color` method is called. ColorElements are
//! categorized by 'order', which denotes the number of dependencies needed to
//! generate a color. For example, a second order element is dependent upon two
//! other colors, while a zeroth order color element is simply a color. These
//! dependencies are expressed through references to other slots in the palette.
//!
////////////////////////////////////////////////////////////////////////////////
use super::element::{Slot, ColorElement};
use super::metadata::Metadata;
use super::format::{PaletteFormat, DEFAULT_FORMAT};
use super::error::{Error, Result};
use color::Color;
use address::Address;
use address;

use std::rc::Rc;
use std::collections::BTreeMap;
use std::collections::btree_map::{Iter, Keys};
use std::u8;
use std::fmt;
use std::result;
use std::mem;



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
	/// # Example
	/// ```rust
	/// # use rampeditor::palette::Palette;
	/// let pal = Palette::new();
	/// ```
	#[inline]
	pub fn new() -> Palette {
		Default::default()
	}

	/// Returns the number of colors in the Palette.
	///
	/// # Example
	/// ```rust
	/// # use rampeditor::palette::Palette;
	/// # use rampeditor::Color;
	/// let mut pal = Palette::new();
	/// assert_eq!(pal.len(), 0);
	///
	/// pal.add_color(Color(1, 2, 3));
	/// assert_eq!(pal.len(), 1);
	/// ```
	#[inline]
	pub fn len(&self) -> usize {
		self.data.len()
	}

	/// Returns the number of addresses still available in the palette.
	/// # Example
	/// ```rust
	/// # use rampeditor::palette::Palette;
	/// # use rampeditor::Color;
	/// let mut pal = Palette::new(); 
	/// // Default palette is maximally sized:
	/// assert_eq!(pal.free_addresses(), 16_581_375); 
	///
	/// pal.add_color(Color(1, 2, 3));
	/// assert_eq!(pal.free_addresses(), 16_581_374);
	/// ```
	#[inline]
	pub fn free_addresses(&self) -> usize {
		self.size_bound() - self.data.len()
	}

	/// Adds a new color to the palette in the nearest valid location after 
	/// the selection cursor and returns its address. Returns an error if the 
	/// palette is full.
	///
	/// # Example
	/// ```rust
	/// # use rampeditor::palette::Palette;
	/// # use rampeditor::Color;
	/// let mut pal = Palette::new();
	/// pal.add_color(Color(255, 0, 0));
	/// pal.add_color(Color(0, 255, 0));
	/// pal.add_color(Color(0, 0, 255));
	///
	/// assert_eq!(pal.len(), 3);
	/// ```
	///
	/// # Example
	/// ```rust
	/// # use rampeditor::palette::{Palette, PaletteBuilder};
	/// # use rampeditor::Color;
	/// # let mut pal = PaletteBuilder::new()
	/// # 	.with_page_count(1)
	/// # 	.with_line_count(1)
	/// # 	.with_column_count(1)	
	/// # 	.create();
	/// # pal.add_color(Color(255, 0, 0));
	/// assert_eq!(pal.free_addresses(), 0);
	/// let result = pal.add_color(Color(0, 0, 0)); // fails...
	/// assert!(result.is_err()); 
	/// ```
	#[inline]
	pub fn add_color(
		&mut self, 
		new_color: Color) 
		-> Result<Address> 
	{
		self.add_element(ColorElement::ZerothOrder {color: new_color})
	}

	/// Returns the color located at the given address, or None if the address 
	/// is invalid or empty.
	///
	/// # Examples
	/// ```rust
	/// # use rampeditor::palette::Palette;
	/// # use rampeditor::Address;
	/// # use rampeditor::Color;
	/// let mut pal = Palette::new();
	/// pal.add_color(Color(255, 0, 0));
	/// pal.add_color(Color(0, 255, 0));
	/// pal.add_color(Color(0, 0, 255));
	///
	/// let red = pal.get_color(Address::new(0, 0, 0)).unwrap();
	/// let blue = pal.get_color(Address::new(0, 0, 1)).unwrap();
	/// let green = pal.get_color(Address::new(0, 0, 2)).unwrap();
	///
	/// assert_eq!(red, Color(255, 0, 0));
	/// assert_eq!(blue, Color(0, 255, 0));
	/// assert_eq!(green, Color(0, 0, 255));
	/// ```
	///
	/// Empty slots are empty:
	/// ```rust
	/// # use rampeditor::palette::Palette;
	/// # use rampeditor::Address;
	/// # use rampeditor::Color;
	/// # let mut pal = Palette::new();
	/// assert!(pal.get_color(Address::new(0, 2, 4)).is_none())
	/// ```
	#[inline]
	pub fn get_color(&self, address: Address) -> Option<Color> {
		self.get_slot(address).and_then(|slot| slot.get_color())
	}

	/// Sets the color at the located address. Returns the old color if it 
	/// succeeds, or none if there was no color at the location. Returns an 
	/// error if the address is invalid, or if the element at the address is a
	/// derived color value.
	pub fn set_color(
		&mut self, 
		address: Address,
		new_color: Color) 
		-> Result<Option<Color>>
	{
		if self.check_address(address) {
			let new_element = ColorElement::ZerothOrder {color: new_color};
			if self.data.contains_key(&address) {
				if let Some(slot) = self.get_slot(address) {
					if slot.get_order() != 0 {
						return Err(Error::CannotSetDerivedColor)
					}
					let old_element = &mut*slot.borrow_mut();
					let old = mem::replace(old_element, new_element);
					return Ok(old.get_color())
				} 
			}
		} 
		Err(Error::InvalidAddress)
	}


	/// Adds a new element to the palette in the nearest valid location after 
	/// the selection cursor and returns its address. Returns an error if the 
	/// palette is full.
	#[inline]
	pub fn add_element(
		&mut self, 
		new_element: ColorElement) 
		-> Result<Address> 
	{
		self.add_slot(Slot::new(new_element))
	}

	/// Sets the element at the located address. Returns the old element if it 
	/// succeeds, or none if there was no element at the location. Returns an 
	/// error if the address is invalid.
	pub fn set_element(
		&mut self, 
		address: Address, 
		new_element: ColorElement) 
		-> Result<Option<ColorElement>> 
	{
		if self.check_address(address) {
			if self.data.contains_key(&address) {
				if let Some(slot) = self.get_slot(address) {
					let old_element = &mut*slot.borrow_mut();
					let old = mem::replace(old_element, new_element);
					return Ok(Some(old));
				}
			}
			self.data.insert(address, Rc::new(Slot::new(new_element)));
			return Ok(None)
		}
		Err(Error::InvalidAddress)
	}

	/// Adds a new slot to the palette in the nearest valid location after the 
	/// selection cursor and returns its address. Returns an error if the 
	/// palette is full.
	#[inline]
	pub fn add_slot(&mut self, new_slot: Slot) -> Result<Address> {
		let address = try!(self.next_free_address_advance_cursor());
		self.data.insert(address, Rc::new(new_slot));
		Ok(address)
	}

	/// Removes a slot from the palette and returns it. Returns None if the slot
	/// doesn't exist in the palette.
	#[inline]
	pub fn remove_slot(&mut self, address: Address) -> Option<Rc<Slot>> {
		self.data.remove(&address)
	}

	/// Returns the slot located at the given address, or `None` if the address
	/// is invalid or empty.
	#[inline]
	pub fn get_slot(&self, address: Address) -> Option<&Rc<Slot>> {
		self.data.get(&address)
	}

	/// Returns the palette's current selection cursor.
	///
	/// # Example
	/// ```rust
	/// # use rampeditor::palette::Palette;
	/// # use rampeditor::{Select, Address};
	/// # use rampeditor::Color;
	/// let mut pal = Palette::new();
	/// assert_eq!(pal.get_cursor(), Select::All);
	///
	/// pal.add_color(Color(1, 2, 3)).ok().unwrap();
	/// assert_eq!(pal.get_cursor(), Select::Address(Address::new(0,0,1)));
	/// ```
	#[inline]
	pub fn get_cursor(&self) -> address::Select {
		self.address_cursor
	}

	/// Sets the palette's selection cursor to the given selection. Does nothing
	/// if the selection is not valid for this palette.
	///
	/// # Example
	/// ```rust
	/// # use rampeditor::palette::Palette;
	/// # use rampeditor::{Select, Address};
	/// # use rampeditor::Color;
	/// # let mut pal = Palette::new();
	/// 
	/// let s = Select::Line {page: 2, line: 3};
	/// pal.set_cursor(s);
	/// assert_eq!(pal.get_cursor(), s);
	/// 
	/// pal.set_cursor(Select::All);
	/// assert_eq!(pal.get_cursor(), Select::All);
	/// ```
	#[inline]
	pub fn set_cursor(&mut self, new_selection: address::Select) {
		if self.check_address(new_selection.base_address()) {
			self.address_cursor = new_selection;
		}
	}

	/// Sets the name for the given selection.
	pub fn set_name<S>(&mut self, selection: address::Select, name: S) 
		where S: Into<String> 
	{
		self.metadata
			.entry(selection)
			.or_insert(Default::default())
			.name = Some(name.into());
	}

	/// Returns an iterator over the palette slots contained in given selection.
	#[inline]
	pub fn select_iter(&self, selection: address::Select) -> SelectIterator {
		SelectIterator::new(self, selection)
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
	fn next_free_address_advance_cursor(&mut self) -> Result<Address> {
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
	fn next_free_address(&self) -> Result<Address> {
		if self.free_addresses() == 0 {
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
		address.page < self.page_count &&
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
}


impl fmt::Display for Palette {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		try!(write!(f, "Palette"));
		if let Some(data) = self.metadata.get(&address::Select::All) {
			try!(write!(f, " {:?}\n", data));
		}
		try!(write!(f, 
			" [{} pages] [wrap {}:{}] [cursor {}] \
			[{} slots free]",
			self.page_count,
			self.line_count,
			self.column_count,
			self.address_cursor,
			self.free_addresses()
		));
		

		try!(write!(f, "\n\tAddress   Color    Order  Name\n"));
		for (&address, ref slot) in self.data.iter() {
			try!(write!(f, "\t{:X}  {:X}  {:<5}  ",
				address,
				slot.borrow().get_color().unwrap_or(Color(0,0,0)),
				slot.borrow().get_order()
			));
			if let Some(data) = self.metadata.get(&address.clone().into()) {
				try!(write!(f, "{:?}\n", data));
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
/// An iterator over the occupied addresses of a palette. The addresses are 
/// returned in order.
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
// SelectIterator
////////////////////////////////////////////////////////////////////////////////
/// An iterator over the selected slots of a palette.
pub struct SelectIterator<'p> {
	inner: Iter<'p, Address, Rc<Slot>>,
	selection: address::Select,
}


impl<'p> SelectIterator<'p> {
	fn new(palette: &'p Palette, selection: address::Select) -> Self {
		SelectIterator {
			inner: palette.data.iter(),
			selection: selection,
		}
	}
}


impl<'p> Iterator for SelectIterator<'p> {
	type Item = Rc<Slot>;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some((&key, value)) = self.inner.next() {
			if self.selection.contains(key) {
				return Some(value.clone());
			}
		}
		None
	}
}


////////////////////////////////////////////////////////////////////////////////
// PaletteBuilder
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates the state of the palette during builder pattern construction.
pub struct PaletteBuilder {
	format: &'static PaletteFormat,
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
	#[allow(unused_mut)]
	pub fn using_format(
		mut self, 
		format: &'static PaletteFormat) 
		-> PaletteBuilder 
	{
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
			address_cursor: self.address_cursor,
			page_count: self.page_count,
			line_count: self.line_count,
			column_count: self.column_count,
			.. Default::default()
		};

		self.format.prepare_new_palette(&mut pal);

		if let Some(name) = self.palette_name {
			pal.set_name(address::Select::All, name)
		}
		pal
	}
}


impl Default for PaletteBuilder {
	fn default() -> Self {
		PaletteBuilder {
			format: DEFAULT_FORMAT,
			address_cursor: address::Select::All,
			page_count: u8::MAX,
			line_count: u8::MAX,
			column_count: u8::MAX,
			palette_name: None,
		}
	}
}


