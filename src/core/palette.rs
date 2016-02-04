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
use super::address::{Address, Select};

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;
use std::collections::BTreeMap;
use std::collections::btree_map::{Iter, Keys};

const DEFAULT_COLUMN_WRAP: u8 = 16;
const DEFAULT_LINE_WRAP: u8 = 16;

////////////////////////////////////////////////////////////////////////////////
// Palette
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single palette.
#[derive(Debug)]
pub struct Palette<'p> {
	// The contents of the palette.
	elements: Vec<PaletteElement<'p>>,
	// A map associating addresses to indices into the elements vector.
	address_map: BTreeMap<Address, usize>,
	// The internal address cursor that is used to track the next available 
	// address.
	address_cursor: Select,
	// The line and column wrapping to use when generating new addresses.
	line_wrap: u8,
	column_wrap: u8,
}


impl<'p> Palette<'p> {
	
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
			elements: Vec::new(),
			address_cursor: Select::All,
			line_wrap: line_wrap,
			column_wrap: column_wrap,
			address_map: BTreeMap::new(),
		}
	}


	/// Returns the number of colors in the Palette.
	pub fn len(&self) -> usize {
		self.elements.len()
	}

	/// Returns the Color associated with the given index.
	pub fn get_color(&self, address: Address) -> Option<Color> {
		self.address_map
			.get(&address)
		    .map(|&i| self.elements[i].borrow().get_color())
	}

	/// Add the given color to the palette.
	pub fn add_color(&mut self, color: Color) {
		self.add_element(ColorElement::ZerothOrder{color: color});
	}

	/// Adds to the Palette a linearly interpolated RGB color ramp of the given 
	/// length between the colors given by their indices in the palette.
	pub fn add_ramp_between(
		&mut self, 
		start_index: usize, 
		end_index: usize, 
		length: u8)
	{
		// Return if ramp would have no colors.
		if length == 0 || start_index == end_index {return;}
		for i in 0..length {
			let p1 = self.get_element(start_index);
			let p2 = self.get_element(end_index);
			// Compute distance between points for this element.
			let factor = (i + 1) as f32 * (1.0 / (length + 1) as f32);
			self.add_element(ColorElement::SecondOrder{
				build: Box::new(move |a, b| {
					// Build color by doing lerp with computed factor.
					lerp_rgb(a.get_color(), b.get_color(), factor)
				}),
				parents: (p1, p2)
			});
		}

	}

	/// Returns an iterator over the (Address, Color) entries of the palette.
	pub fn iter(&'p self) -> PaletteIterator<'p> {
		PaletteIterator::new(self)
	}

	/// Returns and iterator over the colors of the palette in address order.
	pub fn colors(&'p self) -> ColorIterator<'p> {
		ColorIterator::new(self)
	}

	/// Returns and iterator over the addresses of the palette in order.
	pub fn addresses(&'p self) -> AddressIterator<'p> {
		AddressIterator::new(self)
	}

	/// Adds a new element to the palette.
	fn add_element(&mut self, element: ColorElement<'p>) {
		// Add element to the vector.
		self.elements.push(Rc::new(RefCell::new(element)));
		// Find next free address.
		let next_address = self.next_free_address();
		// Update the address map.
		self.address_map.insert(
			next_address, 
			self.elements.len() -1
		);
		// Update the cursor.
		self.address_cursor = next_address.wrapped_next(
			self.line_wrap, 
			self.column_wrap
		).into();
	}

	/// Returns the element stored at the given index.
	fn get_element(&self, index: usize) -> PaletteElement<'p> {
		self.elements[index].clone()
	}

	/// Returns the next available address after the 
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
}


// impl<'p> fmt::Display for Palette<'p> {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
// 		try!(write!(f, "Palette [wrap {}:{}] [select {}]",
// 			self.line_wrap,
// 			self.column_wrap,
// 			self.address_cursor
// 		));

// 		for address in self.addresses() {
// 			try!(write!(f, "\t{}\t{:?}",
// 				address,
// 				self.get_color(address)
// 			));
// 		}
// 		Ok(())
// 	}
// }

////////////////////////////////////////////////////////////////////////////////
// PaletteIterator
////////////////////////////////////////////////////////////////////////////////
/// An iterator over the (Address, Color) entries of a palette. The entries are 
/// returned in address order.
pub struct PaletteIterator<'p> {
	palette: &'p Palette<'p>,
	inner: Iter<'p, Address, usize>
}


impl<'p> PaletteIterator<'p> {
	fn new(palette: &'p Palette<'p>) -> Self {
		PaletteIterator {
			palette: palette,
			inner: palette.address_map.iter()
		}
	}
}


impl<'p> Iterator for PaletteIterator<'p> {
	type Item = (Address, Color);

	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next().map(|(&address, &i)| 
			(address, (*self.palette.elements[i]).borrow().get_color())
		)
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
	fn new(palette: &'p Palette<'p>) -> Self {
		ColorIterator {inner: PaletteIterator::new(palette)}
	}
}


impl<'p> Iterator for ColorIterator<'p> {
	type Item = Color;

	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next().map(|(_, color)| color)
	}
}


////////////////////////////////////////////////////////////////////////////////
// AddressIterator
////////////////////////////////////////////////////////////////////////////////
/// An iterator over the colors of a palette. The colors are returned in address 
/// order.
pub struct AddressIterator<'p> {
	inner: Keys<'p, Address, usize>
}


impl<'p> AddressIterator<'p> {
	fn new(palette: &'p Palette<'p>) -> Self {
		AddressIterator {inner: palette.address_map.keys()}
	}
}


impl<'p> Iterator for AddressIterator<'p> {
	type Item = Address;

	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next().map(|&address| address)
	}
}