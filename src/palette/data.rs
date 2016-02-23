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
//! Defines a structured PaletteData object for storing common data for palette
//! formats.
//!
////////////////////////////////////////////////////////////////////////////////
use super::element::{Slot, ColorElement};
use super::metadata::Metadata;
use super::error::{Error, Result};
use color::Color;
use address::{Address, Group, 
	PageCount, LineCount, ColumnCount, 
	PAGE_MAX, LINE_MAX, COLUMN_MAX
};

use std::rc::Rc;
use std::collections::BTreeMap;
use std::fmt;
use std::result;
use std::mem;

/// Default function for prepare_new_page and prepare_new_line triggers.
#[allow(unused_variables)]
#[inline]
fn no_op(data: &mut PaletteData, group: Group) {}

////////////////////////////////////////////////////////////////////////////////
// PaletteData
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single palette.
pub struct PaletteData {
	/// A map assigning addresses to palette slots.
	pub slotmap: BTreeMap<Address, Rc<Slot>>,
	/// Provided metadata for various parts of the palette.
	pub metadata: BTreeMap<Group, Metadata>,
	/// The internal address cursor that is used to track the next available 
	/// address.
	pub address_cursor: Address,
	/// The number of pages in the palette.
	pub page_count: PageCount,
	/// The default number of lines in each page.
	pub default_line_count: LineCount,
	/// The default number of columns in each line.
	pub default_column_count: ColumnCount,
	/// Called before an element is added to a new page in the palette. The 
	/// expectation is that this will add the appropriate meta data to the 
	/// palette. This will be called before the prepare_new_line function is 
	/// called.
	pub prepare_new_page: fn(&mut PaletteData, Group),
	/// Called before an element is added to a new line in the palette. The 
	/// expectation is that this will add the appropriate meta data to the 
	/// palette.
	pub prepare_new_line: fn(&mut PaletteData, Group),
}


impl PaletteData {
	/// Returns the number of colors in the PaletteData.
	///
	/// # Example
	///
	/// ```rust
	/// use rampeditor::palette::PaletteData;
	/// use rampeditor::Color;
	////
	/// let mut dat: PaletteData = Default::default();
	/// assert_eq!(dat.len(), 0);
	///
	/// dat.add_color(Color(1, 2, 3));
	/// assert_eq!(dat.len(), 1);
	/// ```
	#[inline]
	pub fn len(&self) -> usize {
		self.slotmap.len()
	}

	/// Adds a new color to the palette in the nearest valid location after 
	/// the selection cursor and returns its address. Returns an error if the 
	/// palette is full.
	///
	/// # Example
	///
	/// ```rust
	/// use rampeditor::palette::PaletteData;
	/// use rampeditor::Color;
	/// 
	/// let mut dat: PaletteData = Default::default();
	/// dat.add_color(Color(255, 0, 0));
	/// dat.add_color(Color(0, 255, 0));
	/// dat.add_color(Color(0, 0, 255));
	///
	/// assert_eq!(dat.len(), 3);
	/// ```
	///
	/// # Errors
	///
	/// ```rust
	/// # use rampeditor::palette::PaletteData;
	/// # use rampeditor::Color;
	/// let mut dat: PaletteData = Default::default();
	/// dat.page_count = 1;
	/// dat.default_line_count = 1;
	/// dat.default_column_count = 1;
	/// dat.add_color(Color(0, 0, 0));
	/// let result = dat.add_color(Color(0, 0, 0)); // fails...
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
	///
	/// ```rust
	/// use rampeditor::palette::PaletteData;
	/// use rampeditor::{Address, Color};
	/// 
	/// let mut dat: PaletteData = Default::default();
	/// dat.add_color(Color(255, 0, 0));
	/// dat.add_color(Color(0, 255, 0));
	/// dat.add_color(Color(0, 0, 255));
	///
	/// let red = dat.get_color(Address::new(0, 0, 0)).unwrap();
	/// let blue = dat.get_color(Address::new(0, 0, 1)).unwrap();
	/// let green = dat.get_color(Address::new(0, 0, 2)).unwrap();
	///
	/// assert_eq!(red, Color(255, 0, 0));
	/// assert_eq!(blue, Color(0, 255, 0));
	/// assert_eq!(green, Color(0, 0, 255));
	/// ```
	///
	/// Empty slots are return None:
	///
	/// ```rust
	/// # use rampeditor::palette::PaletteData;
	/// # use rampeditor::{Address, Color};
	/// let dat: PaletteData = Default::default();
	/// assert!(dat.get_color(Address::new(0, 2, 4)).is_none())
	/// ```
	#[inline]
	pub fn get_color(&self, address: Address) -> Option<Color> {
		self.slotmap.get(&address).and_then(|slot| slot.get_color())
	}

	/// Sets the color at the located address. Returns the old color if it 
	/// succeeds, or none if there was no color at the location. Returns an 
	/// error if the address is invalid, or if the element at the address is a
	/// derived color value.
	///
	/// # Example
	///
	/// ```rust
	/// use rampeditor::palette::PaletteData;
	/// use rampeditor::{Address, Color};
	/// 
	/// let mut dat: PaletteData = Default::default();
	/// let fst = Address::new(0, 0, 0);
	/// dat.add_color(Color(255, 0, 0));
	/// dat.set_color(fst, Color(50, 50, 50));
	///
	/// assert_eq!(dat.get_color(fst), Some(Color(50, 50, 50)));
	/// ```
	pub fn set_color(
		&mut self, 
		address: Address,
		new_color: Color) 
		-> Result<Option<Color>>
	{
		if self.check_address(address) {
			let new_element = ColorElement::ZerothOrder {color: new_color};
			if self.slotmap.contains_key(&address) {
				if let Some(slot) = self.slotmap.get(&address) {
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
	/// the group cursor and returns its address. Returns an error if the 
	/// palette is full.
	///
	/// # Example
	///
	/// ```rust
	/// use rampeditor::palette::PaletteData;
	/// use rampeditor::palette::element::ColorElement;
	/// use rampeditor::{Address, Color};
	/// 
	/// let mut dat: PaletteData = Default::default();
	/// let fst = Address::new(0, 0, 0);
	/// let elem = ColorElement::ZerothOrder {color: Color(50, 50, 50)};
	/// dat.add_element(elem);
	///
	/// assert_eq!(dat.get_color(fst), Some(Color(50, 50, 50)));
	/// ```
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
			if self.slotmap.contains_key(&address) {
				if let Some(slot) = self.slotmap.get(&address) {
					let old_element = &mut*slot.borrow_mut();
					let old = mem::replace(old_element, new_element);
					return Ok(Some(old));
				}
			}
			self.slotmap.insert(address, Rc::new(Slot::new(new_element)));
			return Ok(None)
		}
		Err(Error::InvalidAddress)
	}

	/// Adds a new slot to the palette in the nearest valid location after the 
	/// group cursor and returns its address. Returns an error if the 
	/// palette is full.
	///
	/// # Example
	///
	/// ```rust
	/// use rampeditor::palette::PaletteData;
	/// use rampeditor::palette::element::{ColorElement, Slot};
	/// use rampeditor::{Address, Color};
	/// 
	/// let mut dat: PaletteData = Default::default();
	/// let fst = Address::new(0, 0, 0);
	/// let elem = ColorElement::ZerothOrder {color: Color(50, 50, 50)};
	/// let slot = Slot::new(elem);
	/// dat.add_slot(slot);
	///
	/// assert_eq!(dat.get_color(fst), Some(Color(50, 50, 50)));
	/// ```
	#[inline]
	pub fn add_slot(&mut self, new_slot: Slot) -> Result<Address> {
		let address = try!(self.next_free_address_advance_cursor());
		self.slotmap.insert(address, Rc::new(new_slot));
		Ok(address)
	}

	/// Returns the label associated with the given group, or
	/// None if it has no label.
	///
	/// # Example
	///
	/// ```rust
	/// use rampeditor::palette::PaletteData;
	/// use rampeditor::Group;
	/// 
	/// let mut dat: PaletteData = Default::default();
	/// dat.set_label(Group::All, "My Palette");
	///
	/// assert_eq!(dat.get_label(Group::All), Some("My Palette"));
	/// ```
	pub fn get_label(&self, group: Group) -> Option<&str> {
		self.metadata
			.get(&group)
			.and_then(|ref slotmap| slotmap.format_label.as_ref())
			.map(|label| &label[..])
	}

	/// Sets the label for the given group.
	pub fn set_label<S>(
		&mut self, 
		group: Group, 
		format_label: S) 
		where S: Into<String> 
	{
		self.metadata
			.entry(group)
			.or_insert(Default::default())
			.format_label = Some(format_label.into());
	}

	/// Returns the name associated with the given group, or None if it has
	/// no name.
	///
	/// # Example
	///
	/// ```rust
	/// use rampeditor::palette::PaletteData;
	/// use rampeditor::Group;
	/// 
	/// let mut dat: PaletteData = Default::default();
	/// dat.set_name(Group::All, "My Palette");
	///
	/// assert_eq!(dat.get_name(Group::All), Some("My Palette"));
	/// ```
	pub fn get_name(&self, group: Group) -> Option<&str> {
		self.metadata
			.get(&group)
			.and_then(|ref data| data.name.as_ref())
			.map(|name| &name[..])
	}

	/// Sets the name for the given group.
	pub fn set_name<S>(&mut self, group: Group, name: S) 
		where S: Into<String> 
	{
		self.metadata
			.entry(group)
			.or_insert(Default::default())
			.name = Some(name.into());
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
			self.default_line_count, 
			self.default_column_count
		);
		Ok(address)
	}

	/// Returns the next available address after the cursor. Returns an error if
	/// there are no free addresses.
	#[inline]
	fn next_free_address(&mut self) -> Result<Address> {
		let mut address = self.address_cursor;

		// Ensure that the current address has been prepared in case the cursor
		// was moved.
		self.prepare_and_get_line_count(address.page_group());
		self.prepare_and_get_column_count(address.line_group());

		// Loop until we don't see a color.
		while self.slotmap
			.get(&address)
			.and_then(|s| s.get_color())
			.is_some() 
		{
			address = address.wrapped_next(
				self.page_count,
				self.prepare_and_get_line_count(address.page_group()), 
				self.prepare_and_get_column_count(address.line_group())
			);
			// Return an error if we've looped all the way around.
			if address == self.address_cursor {
				return Err(Error::MaxSlotLimitExceeded);
			}
		}
		Ok(address)
	}

	/// Calls the prepare_new_page function and returns the current line count 
	/// for the given group.
	#[inline]
	pub fn prepare_and_get_line_count(&mut self, group: Group) -> LineCount {
		if !self.metadata.contains_key(&group) {
			(self.prepare_new_page)(self, group);
		}
		self.metadata
			.get(&group)
			.map_or(
				self.default_line_count, 
				|ref meta| meta.line_count
			)
	}

	/// Sets the line count for a group.
	pub fn set_line_count(&mut self, group: Group, line_count: LineCount) {
		self.metadata
			.entry(group)
			.or_insert(Default::default())
			.line_count = line_count;
	}

	/// Calls the prepare_new_line function and returns the current column count 
	/// for the given group.
	#[inline]
	pub fn prepare_and_get_column_count(&mut self, group: Group) -> LineCount {
		if !self.metadata.contains_key(&group) {
			(self.prepare_new_line)(self, group);
		}
		self.metadata
			.get(&group)
			.map_or(
				self.default_column_count, 
				|ref meta| meta.column_count
			)
	}

	/// Sets the column count for a group.
	pub fn set_column_count(&mut self, group: Group, column_count: LineCount) {
		self.metadata
			.entry(group)
			.or_insert(Default::default())
			.column_count = column_count;
	}

	/// Returns whether the give address lies within the bounds defined by the 
	/// wrapping and max page settings for the palette.
	#[inline]
	pub fn check_address(&mut self, address: Address) -> bool {
		address.page < self.page_count &&
		address.line < self.prepare_and_get_line_count(address.page_group()) &&
		address.column < self.prepare_and_get_column_count(address.line_group())
	}
}


impl fmt::Debug for PaletteData {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		write!(f, "PaletteData {{ \
			slotmap: {:#?}, \
			metadata: {:#?}, \
			address_cursor: {:#?}, \
			page_count: {:#?}, \
			default_line_count: {:#?}, \
			default_column_count: {:#?} }}",
			self.slotmap,
			self.metadata,
			self.address_cursor,
			self.page_count,
			self.default_line_count,
			self.default_column_count
		)
	}
}


impl fmt::Display for PaletteData {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		if let Some(data) = self.metadata.get(&Group::All) {
			try!(write!(f, " {}\n", data));
		}
		try!(write!(f, 
			" [{} pages] [default wrap {}:{}] [cursor {}]",
			self.page_count,
			self.default_line_count,
			self.default_column_count,
			self.address_cursor
		));

		try!(write!(f, "\n\tAddress   Color    Order  Name\n"));
		let mut cur_page_group = Group::All;
		for (&address, ref slot) in self.slotmap.iter() {
			if cur_page_group != address.page_group() {
				match self.metadata.get(&address.page_group()) {
					Some(meta) => try!(writeln!(f, "Page {} - {}", 
						address.page_group(), 
						meta)
					),
					None => try!(writeln!(f, "Page {}", 
						address.page_group())
					)
				}
			};
			cur_page_group = address.page_group();
			if let Some(meta) = self.metadata.get(&address.line_group()) {
				try!(write!(f, "\t{}", meta));
			}
			try!(write!(f, "\t{:X}  {:X}  {:<5}  ",
				address,
				slot.borrow().get_color().unwrap_or(Color(0,0,0)),
				slot.borrow().get_order()
			));
		}
		Ok(())
	}
}


impl Default for PaletteData {
	fn default() -> Self {
		PaletteData {
			slotmap: BTreeMap::new(),
			metadata: BTreeMap::new(),
			address_cursor: Default::default(),
			page_count: PAGE_MAX,
			default_line_count: LINE_MAX,
			default_column_count: COLUMN_MAX,
			prepare_new_page: no_op,
			prepare_new_line: no_op,
		}
	}
}