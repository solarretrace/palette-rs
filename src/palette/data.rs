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
use address::{Address, Group};
use address;

use std::rc::Rc;
use std::collections::BTreeMap;
use std::collections::btree_map::{Iter, Keys};
use std::u8;
use std::u16;
use std::fmt;
use std::result;
use std::mem;



////////////////////////////////////////////////////////////////////////////////
// PaletteData
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single palette.
#[derive(Debug)]
pub struct PaletteData {
	/// A map assigning addresses to palette slots.
	pub slotmap: BTreeMap<Address, Rc<Slot>>,
	/// Provided metadata for various parts of the palette.
	pub metadata: BTreeMap<Group, Metadata>,
	/// The internal address cursor that is used to track the next available 
	/// address.
	pub address_cursor: Group,
	/// The number of pages in the palette.
	pub page_count: u16,
	/// The number of lines in each page.
	pub line_count: u8,
	/// The number of columns in each line.
	pub column_count: u8,
}


impl PaletteData {
	/// Returns the number of colors in the PaletteData.
	///
	/// # Example
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

	/// Returns the number of addresses still available in the palette.
	/// # Example
	/// ```rust
	/// use rampeditor::palette::PaletteData;
	/// use rampeditor::Color;
	////
	/// let mut dat: PaletteData = Default::default(); 
	/// // Default palette is maximally sized:
	/// assert_eq!(dat.free_addresses(), 16_581_375); 
	///
	/// dat.add_color(Color(1, 2, 3));
	/// assert_eq!(dat.free_addresses(), 16_581_374);
	/// ```
	#[inline]
	pub fn free_addresses(&self) -> usize {
		self.size_bound() - self.slotmap.len()
	}

	/// Adds a new color to the palette in the nearest valid location after 
	/// the selection cursor and returns its address. Returns an error if the 
	/// palette is full.
	///
	/// # Example
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
	/// # Example
	/// ```rust
	/// # use rampeditor::palette::PaletteData;
	/// # use rampeditor::Color;
	/// let mut dat: PaletteData = Default::default();
	/// dat.page_count = 1;
	/// dat.line_count = 1;
	/// dat.column_count = 1;
	/// # dat.add_color(Color(255, 0, 0));
	/// assert_eq!(dat.free_addresses(), 0);
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
	/// ```rust
	///  use rampeditor::palette::PaletteData;
	///  use rampeditor::{Address, Color};
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
	/// Empty slots are empty:
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
	#[inline]
	pub fn add_slot(&mut self, new_slot: Slot) -> Result<Address> {
		let address = try!(self.next_free_address_advance_cursor());
		self.slotmap.insert(address, Rc::new(new_slot));
		Ok(address)
	}

	/// Returns the label associated with the given group, or
	/// None if it has no label.
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

	/// Returns an iterator over the palette slots contained in given group.
	#[inline]
	pub fn select_iter(&self, group: Group) -> SelectIterator {
		SelectIterator::new(self, group)
	}

	/// Returns an iterator over the (Address, Color) entries of the palette.
	#[inline]
	pub fn iter(&self) -> DataIterator {
		DataIterator::new(self)
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

	/// Returns whether the format's prepare function has been called for the 
	/// given group.
	fn is_initialized(&self, group: Group) -> bool {
		self.metadata
			.get(&group)
			.map_or(false, |ref slotmap| slotmap.initialized)
	}

	/// Sets the format preparation flag for the group.
	pub fn set_initialized(&mut self, group: Group, value: bool) {
		self.metadata
			.entry(group)
			.or_insert(Default::default())
			.initialized = value;
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
		while self.slotmap.get(&address).and_then(|s| s.get_color()).is_some() {
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


impl fmt::Display for PaletteData {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		if let Some(data) = self.metadata.get(&Group::All) {
			try!(write!(f, " {}\n", data));
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
		for (&address, ref slot) in self.slotmap.iter() {
			try!(write!(f, "\t{:X}  {:X}  {:<5}  ",
				address,
				slot.borrow().get_color().unwrap_or(Color(0,0,0)),
				slot.borrow().get_order()
			));
			if let Some(slotmap) = self.metadata.get(&address.clone().into()) {
				try!(write!(f, "{:?}\n", slotmap));
			} else {
				try!(write!(f, "-\n"));
			}
		}
		Ok(())
	}
}


impl Default for PaletteData {
	fn default() -> Self {
		PaletteData {
			slotmap: BTreeMap::new(),
			metadata: BTreeMap::new(),
			address_cursor: Group::All,
			page_count: u16::MAX,
			line_count: u8::MAX,
			column_count: u8::MAX,
		}
	}
}



////////////////////////////////////////////////////////////////////////////////
// DataIterator
////////////////////////////////////////////////////////////////////////////////
/// An iterator over the (Address, Color) entries of a palette. The entries are 
/// returned in address order.
pub struct DataIterator<'p> {
	inner: Iter<'p, Address, Rc<Slot>>
}


impl<'p> DataIterator<'p> {
	fn new(palette: &'p PaletteData) -> Self {
		DataIterator {
			inner: palette.slotmap.iter()
		}
	}
}


impl<'p> Iterator for DataIterator<'p> {
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
	inner: DataIterator<'p>
}


impl<'p> ColorIterator<'p> {
	fn new(palette: &'p PaletteData) -> Self {
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
	fn new(palette: &'p PaletteData) -> Self {
		AddressIterator {inner: palette.slotmap.keys()}
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
	group: Group,
}


impl<'p> SelectIterator<'p> {
	fn new(palette: &'p PaletteData, group: Group) -> Self {
		SelectIterator {
			inner: palette.slotmap.iter(),
			group: group,
		}
	}
}


impl<'p> Iterator for SelectIterator<'p> {
	type Item = Rc<Slot>;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some((&key, value)) = self.inner.next() {
			if self.group.contains(key) {
				return Some(value.clone());
			}
		}
		None
	}
}

