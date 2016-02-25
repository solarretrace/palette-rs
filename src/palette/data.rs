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
use super::element::Slot;
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

/// Default function for prepare_new_page and prepare_new_line triggers.
#[allow(unused_variables)]
#[inline]
fn no_op(data: &mut PaletteData, group: Group) {}


////////////////////////////////////////////////////////////////////////////////
// Metadata
////////////////////////////////////////////////////////////////////////////////
/// Provides metadata about palette data.
#[derive(Debug, Default)]
pub struct Metadata {
	/// A format-generated label for the item.
	pub format_label: Option<String>,
	/// A user-provided name for the item.
	pub name: Option<String>,
	/// An override to the default line count for this group.
	pub line_count: LineCount,
	/// An override to the default column count for this group.
	pub column_count: ColumnCount,
}

impl fmt::Display for Metadata {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		try!(match (self.name.as_ref(), self.format_label.as_ref()) {
			(Some(name), Some(label)) => write!(f, "\"{}\" ({})", name, label),
			(None, Some(label)) => write!(f, "({})", label),
			(Some(name), None) => write!(f, "\"{}\"", name),
			_ => Ok(())
		});
		write!(f, " [Lines: {}] [Columns: {}]", 
			self.line_count, 
			self.column_count)
	}
}



////////////////////////////////////////////////////////////////////////////////
// PaletteData
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single palette.
pub struct PaletteData {
	/// A map assigning addresses to palette slots.
	pub slotmap: BTreeMap<Address, Rc<Slot>>,
	/// Provided metadata for various parts of the palette.
	pub metadata: BTreeMap<Group, Metadata>,
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

	/// Returns a weak reference to the slot located at the given address, or 
	/// None if the address is invalid or empty.
	pub fn get_slot(&self, address: Address) -> Option<Rc<Slot>> {
		self.slotmap.get(&address).map(|slot| slot.clone())
	}

	/// Returns a weak reference to the slot located at the given address. If 
	/// the address is empty, a new slot will be created an a weak reference 
	/// will be returned. Returns None if the address is invalid.
	///
	/// # Example
	///
	/// ```rust
	/// use rampeditor::palette::PaletteData;
	/// use rampeditor::{Address, Color};
	/// 
	/// let mut dat: PaletteData = Default::default();
	/// let slot = dat.get_or_create_slot(Address::new(1, 1, 1))
	/// 	.ok()
	/// 	.unwrap()
	///		.upgrade()
	///		.unwrap(); // Create slot with default Color and unwrap weak ref.
	///
	/// assert_eq!(slot.get_color(), Some(Default::default()));
	/// ```
	pub fn get_or_create_slot(
		&mut self, 
		address: Address) 
		-> Result<Rc<Slot>> 
	{
		if let Some(slot) = self.get_slot(address) {
			Ok(slot)
		} else {
			try!(self.prepare_address(address));
			let new_slot = Rc::new(Slot::new(Default::default()));
			self.slotmap.insert(address, new_slot.clone());
			Ok(new_slot)
		}
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

	/// Returns the next free address after the given address. And error will be
	/// returned if there are no more free addresses.
	#[inline]
	fn first_free_address_after(
		&mut self, 
		starting_address: Address) 
		-> Result<Address> 
	{
		let mut address = starting_address;
		try!(self.prepare_address(address));

		// Loop until we don't see a color.
		while self.slotmap
			.get(&address)
			.and_then(|s| s.get_color())
			.is_some() 
		{
			address = address.wrapping_add(
				1,
				self.page_count,
				self.get_line_count(address.page_group()), 
				self.get_column_count(address.line_group())
			);
			// Return an error if we've looped all the way around.
			if address == starting_address {
				return Err(Error::MaxSlotLimitExceeded);
			}
		}
		Ok(address)
	}

	/// Calls the prepare_new_page function and returns the current line count 
	/// for the given group.
	#[inline]
	fn get_line_count(&mut self, group: Group) -> LineCount {
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
	fn get_column_count(&mut self, group: Group) -> ColumnCount {
		self.metadata
			.get(&group)
			.map_or(
				self.default_column_count, 
				|ref meta| meta.column_count
			)
	}

	/// Sets the column count for a group.
	pub fn set_column_count(
		&mut self, 
		group: Group, 
		column_count: ColumnCount) 
	{
		self.metadata
			.entry(group)
			.or_insert(Default::default())
			.column_count = column_count;
	}

	/// Returns whether the give address lies within the bounds defined by the 
	/// wrapping and max page settings for the palette.
	#[inline]
	fn check_address(&mut self, address: Address) -> bool {
		address.page < self.page_count &&
		address.line < self.get_line_count(address.page_group()) &&
		address.column < self.get_column_count(address.line_group())
	}


	pub fn prepare_address(&mut self, address: Address) -> Result<()> {
		let default_line_count = self.default_line_count;
		let default_column_count = self.default_column_count;
		let page_group = address.page_group();
		let line_group = address.line_group();
		if !self.metadata.contains_key(&page_group) {
			self.set_line_count(page_group, default_line_count);
			(self.prepare_new_page)(self, page_group);
		}
		if !self.metadata.contains_key(&line_group) {
			self.set_column_count(line_group, default_column_count);
			(self.prepare_new_line)(self, line_group);
		}
		
		if self.check_address(address) {
			Ok(())
		} else {
			Err(Error::InvalidAddress(address))
		}
	}

	/// Retrieves n target addresses after starting_address from the palette. If 
	/// overwriting, the addresses will be contiguous and possible occupied. 
	/// Otherwise, they will be in order and empty. 
	pub fn retrieve_targets(
		&mut self, 
		n: usize, 
		starting_address: Address,
		overwrite: bool)
		-> Result<Vec<Address>>
	{
		let mut targets = Vec::new();
		let mut next = starting_address;

		if overwrite { // Get contiguous block.
			for i in 0..n {
				try!(self.prepare_address(next));
				targets.push(next);
				next = next.wrapping_add(
					1,
					self.page_count,
					self.get_line_count(next.page_group()),
					self.get_column_count(next.line_group()),
				);
			}
		} else { // Find n free addresses.
			try!(self.prepare_address(next));

			// Check if the starting address is empty.
			if next == starting_address && 
				self.slotmap.get(&next).and_then(|s| s.get_color()).is_none() 
			{
				targets.push(next);
			}
			while targets.len() < n {
				next = next.wrapping_add(
					1,
					self.page_count,
					self.get_line_count(next.page_group()),
					self.get_column_count(next.line_group()),
				);
				next = try!(self.first_free_address_after(next));
				targets.push(next);
			}
		}

		Ok(targets)
	}
}


impl fmt::Debug for PaletteData {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		write!(f, "PaletteData {{ \
			slotmap: {:#?}, \
			metadata: {:#?}, \
			page_count: {:#?}, \
			default_line_count: {:#?}, \
			default_column_count: {:#?} }}",
			self.slotmap,
			self.metadata,
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
			" [{} pages] [default wrap {}:{}]",
			self.page_count,
			self.default_line_count,
			self.default_column_count
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
			page_count: PAGE_MAX,
			default_line_count: LINE_MAX,
			default_column_count: COLUMN_MAX,
			prepare_new_page: no_op,
			prepare_new_line: no_op,
		}
	}
}