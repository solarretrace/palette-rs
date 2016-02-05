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

const DEFAULT_MAX_PAGE_COUNT: u8 = 16;
const DEFAULT_COLUMN_WRAP: u8 = 16;
const DEFAULT_LINE_WRAP: u8 = 16;


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
	pub fn new() -> Self {
		Palette::with_wrapping(
			DEFAULT_LINE_WRAP,
			DEFAULT_COLUMN_WRAP
		)
	}

	/// Constructs a new, empty Palette with the given values for line and 
	/// column wrapping settings.
	///
    /// # Example
    /// ```rust
    /// # use rampeditor::core::palette::Palette;
    /// # use rampeditor::core::color::Color;
    /// # use rampeditor::core::address::Address;
    /// // Create a palette that wraps at line 10 and column 4:
    /// let mut pal = Palette::with_wrapping(10, 4);
    ///
    /// // Add many colors to the palette:
    /// for i in 0..100 {
    ///    pal.add_color(Color(i+1, 0, 0));
    /// }	
    ///
    /// // The colors are placed in the appropriate places:
    /// let a100 = Address::new(2, 4, 3); // 100th color halfway down 3rd page.
    /// assert_eq!(pal.get_color(a100).unwrap(), Color(100, 0, 0));
    ///
    /// let a41 = Address::new(1, 0, 0); // 41st color on new page.
    /// assert_eq!(pal.get_color(a41).unwrap(), Color(41, 0, 0));
    /// ```
	pub fn with_wrapping(line_wrap: u8, column_wrap: u8) -> Self {
		Palette {
			address_cursor: address::Select::All,
			max_page_count: DEFAULT_MAX_PAGE_COUNT,
			line_wrap: line_wrap,
			column_wrap: column_wrap,
			address_map: BTreeMap::new(),
		}
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
	pub fn add_color(&mut self, color: Color) -> Address {
		let address = self.next_free_address_advance_cursor();
		let element = Rc::new(RefCell::new(ColorElement::ZerothOrder {
			color: color
		}));
		self.address_map.insert(address, element);

		// Update the cursor.
		self.address_cursor = address.wrapped_next(
			self.line_wrap, 
			self.column_wrap
		).into();

		address
	}

	/// Adds to the Palette a linearly interpolated RGB color ramp of the given 
	/// length between the colors given by their indices in the palette.
	pub fn add_ramp_between(
		&mut self, 
		start_address: Address, 
		end_address: Address, 
		length: u8)
	{
		// Return if ramp would have no colors.
		if length == 0 || start_address == end_address {return;}
		for i in 0..length {
			let p1 = self.address_map.get(&start_address).unwrap().clone();
			let p2 = self.address_map.get(&end_address).unwrap().clone();
			
			// Compute distance between points for this element.
			let factor = (i + 1) as f32 * (1.0 / (length + 1) as f32);

			let address = self.next_free_address_advance_cursor();
			let element = Rc::new(RefCell::new(ColorElement::SecondOrder{
				build: Box::new(move |a, b| {
					// Build color by doing lerp with computed factor.
					lerp_rgb(a.get_color(), b.get_color(), factor)
				}),
				parents: (p1, p2)
			}));
			self.address_map.insert(address, element);
		}
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
	fn next_free_address(&self) -> Address {
		let mut next_address = self.address_cursor.base_address();
		while self.address_map.contains_key(&next_address) {
			next_address = next_address.wrapped_next(
				self.line_wrap, 
				self.column_wrap
			);
		}
		next_address
	}

	/// Returns the next available address after the cursor, and also advances
	/// the cursor to the next (wrapped) address.
	fn next_free_address_advance_cursor(&mut self) -> Address {
		let next_address = self.next_free_address();
		// Update the cursor.
		self.address_cursor = next_address.wrapped_next(
			self.line_wrap, 
			self.column_wrap
		).into();
		next_address
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

impl From<address::Error> for Error {
	fn from(address_error: address::Error) -> Self {
		match address_error {
			address::Error::NoNextAddress 
				=> Error::MaxElementLimitExceeded
		}
	}
}
