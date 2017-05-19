// The MIT License (MIT)
// 
// Copyright (c) 2017 Skylor R. Schermer
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
//! Defines a structured Data object for storing common data for palette
//! formats.
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use address::{
	Address,
	Reference,
	Page, Line, Column, 
	PAGE_MAX, LINE_MAX, COLUMN_MAX,
};
use cell::Cell;
use expression::Expression;
use result::{
	Error,
	Result,
};

// Non-local imports.
use color::Color;

// Standard imports.
use std::collections::{
	BTreeMap,
	BTreeSet,
	HashMap,
};
use std::rc::Rc;
use std::fmt;
use std::mem;



/// Default function for `prepare_new_page` and `prepare_new_line` triggers.
#[allow(unused_variables)]
fn no_op(_: &mut Data, _: &Reference) {}


////////////////////////////////////////////////////////////////////////////////
// MetaData
////////////////////////////////////////////////////////////////////////////////
/// Provides metadata about palette data.
#[derive(Debug, Default)]
pub struct MetaData {
	/// A format-generated label for the item.
	pub format_label: Option<String>,

	/// A user-provided name for the item.
	pub name: Option<String>,
	
	/// An override to the default line count for this group.
	pub line_count: Line,
	
	/// An override to the default column count for this group.
	pub column_count: Column,
}

impl fmt::Display for MetaData {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		match (self.name.as_ref(), self.format_label.as_ref()) {
			(Some(name), Some(label)) => write!(f, "\"{}\" ({})", name, label),
			(None,		 Some(label)) => write!(f, "({})", label),
			(Some(name), None)		  => write!(f, "\"{}\"", name),
			_						  => Ok(())
		}?;
		
		write!(f, " [Lines: {}] [Columns: {}]", 
			self.line_count, 
			self.column_count
		)
	}
}



////////////////////////////////////////////////////////////////////////////////
// Data
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single palette's operation-relevant data.
pub struct Data {
	/// A map assigning addresses to `Palette` cells.
	pub cells: BTreeMap<Address, Rc<Cell>>,

	/// A map assigning references to names.
	pub names: HashMap<String, Reference>,

	/// A map assigning metadata to references.
	pub metadata: HashMap<Reference, MetaData>,

	/// The maximum number of pages in the `Palette`.
	pub maximum_page_count: Page,

	/// The default number of lines in each page.
	pub default_line_count: Line,

	/// The default number of columns in each line.
	pub default_column_count: Column,

	/// Called before a `Cell` is added to a new page in the palette. The 
	/// expectation is that this will add the appropriate meta data to the 
	/// palette. This will be called before the prepare_new_line function is 
	/// called.
	pub prepare_new_page: fn(&mut Data, &Reference),
	
	/// Called before an expression is added to a new line in the palette. The 
	/// expectation is that this will add the appropriate meta data to the 
	/// palette.
	pub prepare_new_line: fn(&mut Data, &Reference),
}


impl Data {
	/// Returns the number of colors in the Data.
	pub fn len(&self) -> usize {
		self.cells.len()
	}

	/// Returns whether there are any `Cell`s in the `Data`.
	pub fn is_empty(&self) -> bool {
		self.cells.is_empty()
	}

	/// Returns a reference to the cell located at the given address, or None if
	/// the address is invalid or empty.
	pub fn get_cell(&self, address: Address) -> Option<Rc<Cell>> {
		self.cells.get(&address).map(Clone::clone)
	}

	/// Returns a reference to the cell located at the given address. If 
	/// the address is empty, a new cell will be created an a weak reference 
	/// will be returned. Returns None if the address is invalid.
	///
	/// # Example
	///
	/// ```rust
	/// use palette::data::Data;
	/// use palette::{Address, Color};
	/// 
	/// let mut dat: Data = Default::default();
	/// let cell = dat.create_cell(Address::new(1, 1, 1))
	/// 	.ok()
	/// 	.unwrap(); // Create empty `Cell` and unwrap it.
	/// ```
	pub fn create_cell(&mut self, address: Address) -> Result<Rc<Cell>> {
		if self.cells.contains_key(&address) {
			Err(Error::AddressInUse(address))
		} else {
			self.prepare_address(address)?;
			let new_cell = Rc::new(Cell::new(Default::default()));
			self.cells.insert(address, new_cell.clone());
			Ok(new_cell)
		}
	}


	/// Removes the expression at the given address from the palette. Returns
	/// the removed expression, or an error if the given address is empty.
	pub fn remove_cell(&mut self, address: Address) -> Result<Expression> {
		// Remove cell from cells.
		let cell = self.cells
			.remove(&address)
			.ok_or(Error::EmptyAddress(address))?;

		// Extract Expression and discard wrappers.
		let expr = mem::replace(&mut *cell.borrow_mut(), Default::default());
		Ok(expr)
	}

	/// Returns the label associated with the given group, or
	/// None if it has no label.
	///
	/// # Example
	///
	/// ```rust
	/// use palette::data::Data;
	/// use palette::address::Reference;
	/// 
	/// let mut dat: Data = Default::default();
	/// dat.set_label(Reference::all(), "My Palette");
	///
	/// assert_eq!(dat.get_label(Reference::all()), Some("My Palette"));
	/// ```
	pub fn get_label(&self, group: Reference) -> Option<&str> {
		self.metadata
			.get(&group)
			.and_then(|ref slotmap| slotmap.format_label.as_ref())
			.map(|label| &label[..])
	}

	/// Sets the label for the given group.
	pub fn set_label<S>(
		&mut self, 
		group: Reference, 
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
	/// ```rust
	/// use palette::data::Data;
	/// use palette::address::Reference;
	/// 
	/// let mut dat: Data = Default::default();
	/// dat.set_name(Reference::all(), "My Palette");
	///
	/// assert_eq!(dat.get_name(Reference::all()), Some("My Palette"));
	/// ```
	pub fn get_name(&self, group: Reference) -> Option<&str> {
		self.metadata
			.get(&group)
			.and_then(|ref data| data.name.as_ref())
			.map(|name| &name[..])
	}

	/// Sets the name for the given group.
	pub fn set_name<S>(&mut self, group: Reference, name: S) 
		where S: Into<String> 
	{
		self.metadata
			.entry(group)
			.or_insert(Default::default())
			.name = Some(name.into());
	}

	/// Returns the next free address after the given address. And error will be
	/// returned if there are no more free addresses.
	pub fn first_free_address_after(
		&mut self, 
		starting_address: Address) 
		-> Result<Address> 
	{
		let mut address = starting_address;
		self.prepare_address(address)?;

		// Loop until we don't see a color.
		while self.cells
			.get(&address)
			.and_then(|s| s.color())
			.is_some() 
		{
			address = address.wrapping_step(
				1,
				self.maximum_page_count,
				self.get_line_count(&Reference::page_of(&address)), 
				self.get_column_count(&Reference::line_of(&address))
			);
			// Return an error if we've looped all the way around.
			if address == starting_address {
				return Err(Error::MaxCellLimitExceeded);
			}
		}
		Ok(address)
	}

	/// Calls the prepare_new_page function and returns the current line count 
	/// for the given group.
	fn get_line_count(&mut self, group: &Reference) -> Line {
		self.metadata
			.get(group)
			.map_or(self.default_line_count, |ref meta| meta.line_count)
	}

	/// Sets the line count for a group.
	pub fn set_line_count(&mut self, group: Reference, line_count: Line) {
		self.metadata
			.entry(group)
			.or_insert(Default::default())
			.line_count = line_count;
	}

	/// Calls the prepare_new_line function and returns the current column count 
	/// for the given group.
	fn get_column_count(&mut self, group: &Reference) -> Column {
		self.metadata
			.get(&group)
			.map_or(self.default_column_count, |ref meta| meta.column_count)
	}

	/// Sets the column count for a group.
	pub fn set_column_count(
		&mut self, 
		group: Reference, 
		column_count: Column) 
	{
		self.metadata
			.entry(group)
			.or_insert(Default::default())
			.column_count = column_count;
	}

	/// Returns whether the give address lies within the bounds defined by the 
	/// wrapping and max page settings for the palette.
	fn check_address(&mut self, address: Address) -> bool {
		address.page < self.maximum_page_count &&
		address.line < self.get_line_count(&Reference::page_of(&address)) &&
		address.column < self.get_column_count(&Reference::line_of(&address))
	}

	/// Prepares an address by calling the palette format's metadata functions.
	/// This function must be called on any address that is first in a new line
	/// in order to ensure the palette wraps properly.
	fn prepare_address(&mut self, address: Address) -> Result<()> {
		let default_line_count = self.default_line_count;
		let default_column_count = self.default_column_count;
		let page_group = Reference::page_of(&address);
		let line_group = Reference::line_of(&address);

		if !self.metadata.contains_key(&page_group) {
			self.set_line_count(page_group.clone(), default_line_count);
			(self.prepare_new_page)(self, &page_group);
		}

		if !self.metadata.contains_key(&line_group) {
			self.set_column_count(line_group.clone(), default_column_count);
			(self.prepare_new_line)(self, &line_group);
		}
		
		if self.check_address(address) {
			Ok(())
		} else {
			Err(Error::InvalidAddress(address))
		}
	}

	/// Retrieves n target addresses after starting_address from the palette. If 
	/// overwrite is true, the addresses may potentially contain expressions. 
	/// Otherwise, they will be empty. Addresses provided in the exclude list 
	/// will be skipped. Returns an error if more targets are requested than are
	/// available in the palette.
	pub fn find_targets(
		&mut self, 
		n: usize, 
		starting_address: Address,
		overwrite: bool,
		exclude: Option<Vec<Address>>)
		-> Result<Vec<Address>>
	{
		let mut targets = BTreeSet::new();
		let mut next = starting_address;

		if overwrite { // Get overwrite block.
			while targets.len() < n {
				self.prepare_address(next)?;
				if targets.contains(&next) {
					return Err(Error::MaxCellLimitExceeded);
				}
				// Add the target if it's not in the exclude list.
				if !exclude.clone().map_or(false, |ex| ex.contains(&next)) {
					targets.insert(next);
				}
				next = next.wrapping_step(
					1,
					self.maximum_page_count,
					self.get_line_count(&Reference::page_of(&next)),
					self.get_column_count(&Reference::line_of(&next)),
				);
			}
		} else { // Find n free addresses.
			self.prepare_address(next)?;

			// Check if the starting address is empty.
			if next == starting_address && 
				self.cells.get(&next).and_then(|s| s.color()).is_none() &&
				!exclude.clone().map_or(false, |ex| ex.contains(&next))
			{
				targets.insert(next);
			}
			
			while targets.len() < n {
				next = next.wrapping_step(
					1,
					self.maximum_page_count,
					self.get_line_count(&Reference::page_of(&next)),
					self.get_column_count(&Reference::line_of(&next)),
				);
				next = self.first_free_address_after(next)?;
				// Add the target if it's not in the exclude list.
				if !exclude.clone().map_or(false, |ex| ex.contains(&next)) {
					targets.insert(next);
				}
			}
		}

		Ok(targets.into_iter().collect())
	}
}


impl fmt::Debug for Data {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Data {{ \
			cells: {:#?}, \
			names: {:#?}, \
			maximum_page_count: {}, \
			default_line_count: {}, \
			default_column_count: {}",
			self.cells,
			self.names,
			self.maximum_page_count,
			self.default_line_count,
			self.default_column_count,
		)
	}
}


impl fmt::Display for Data {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(data) = self.metadata.get(&Reference::all()) {
			write!(f, "{} ", data)?;
		}
		write!(f, 
			"[{} pages] [{} expressions] [default wrap {}:{}]\n",
			self.maximum_page_count,
			self.len(),
			self.default_line_count,
			self.default_column_count
		)?;

		let mut cur_page_group = Reference::all();
		let mut cur_line_group = Reference::all();
		for (&address, ref cell) in self.cells.iter() {
			if cur_page_group != Reference::page_of(&address) {
				match self.metadata.get(&Reference::page_of(&address)) {
					Some(meta) => writeln!(f, "Page {} - {}", 
						Reference::page_of(&address), 
						meta)?
					,
					None => writeln!(f, "Page {}", Reference::page_of(&address))?,
				}
			};
			cur_page_group = Reference::page_of(&address);
			if cur_line_group != Reference::line_of(&address) {
				if let Some(meta) = self.metadata.get(&Reference::line_of(&address)) {
					write!(f, "\t{}\n", meta)?;
				}
				cur_line_group = Reference::line_of(&address);
				write!(f, "\tAddress   Color\n")?;
			}

			writeln!(f, "\t{:X}  {:X}",
				address,
				cell.borrow().color().unwrap_or(Color::new(0,0,0)))?;
		}
		Ok(())
	}
}


impl Default for Data {
	fn default() -> Self {
		Data {
			cells: BTreeMap::new(),
			names: HashMap::new(),
			metadata: HashMap::new(),
			maximum_page_count: PAGE_MAX,
			default_line_count: LINE_MAX,
			default_column_count: COLUMN_MAX,
			prepare_new_page: no_op,
			prepare_new_line: no_op,
		}
	}
}