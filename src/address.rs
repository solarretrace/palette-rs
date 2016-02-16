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

//! Provides a Page:Line:Column addressing object for organizing the palette.
use std::fmt;

////////////////////////////////////////////////////////////////////////////////
// Address
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single address name.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone, Copy, Default)]
pub struct Address {
	/// The page of the Address.
	pub page: u8,
	/// The line of the Address.
	pub line: u8,
	/// The column of the Address.
	pub column: u8,
}

impl Address {
	/// Creates a new Address.
	pub fn new(page: u8, line: u8, column: u8) -> Self {
		Address {
			page: page,
			line: line,
			column: column,
		}
	}

	/// Returns the next address, assuming the given wrapping parameters.
	///
	/// # Example
	/// ```rust
	/// # use rampeditor::Address;
	/// let a = Address::new(0, 9, 9);
	/// let b = a.wrapped_next(10, 10, 10);
	/// 
	/// assert_eq!(b, Address::new(1, 0, 0));
	/// ```
	pub fn wrapped_next(
		&self, 
		pages: u8,
		lines: u8, 
		columns: u8) 
		-> Address
	{
		let mut next = Address::new(
			self.page,
			self.line,
			self.column.wrapping_add(1)
		);
		// Check for column wrap.
		if next.column % columns == 0 { 
			next.column = 0;
			next.line = next.line.wrapping_add(1);
			// Check for line wrap.
			if next.line % lines == 0 {
				next.line = 0;
				next.page = next.page.wrapping_add(1);
				if next.page >= pages {next.page = 0;}
			}
		}
		next
	}
}


impl fmt::Display for Address {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}:{}:{}", self.page, self.line, self.column)
	}
}


impl fmt::UpperHex for Address {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:02X}:{:02X}:{:02X}", self.page, self.line, self.column)
	}
}


impl fmt::LowerHex for Address {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:02x}:{:02x}:{:02x}", self.page, self.line, self.column)
	}
}


impl Into<Select> for Address {
	fn into(self) -> Select {
		Select::Address(self)
	}
}


impl<'a> Into<Select> for &'a Address {
	fn into(self) -> Select {
		Select::Address(*self)
	}
}



////////////////////////////////////////////////////////////////////////////////
// Select
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates the selection of a single address, line, page, or palette.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone, Copy)]
pub enum Select {
	/// A single address selection.
	Address(Address),
	/// A single line selection.
	Line {
		/// The page of the selection.
		page: u8, 
		/// The line of the selection.
		line: u8
	},
	/// A single page selection.
	Page {
		/// The page of the selection.
		page: u8
	},
	/// A full palette selection.
	All,
}


impl Select {
	/// Returns the first address located within the selection.
	pub fn base_address(&self) -> Address {
		match *self {
			Select::Address(addr) => addr,
			Select::Line {page, line} => Address::new(page, line, 0),
			Select::Page {page} => Address::new(page, 0, 0),
			Select::All => Address::new(0, 0, 0),
		}
	}

	/// Returns whether the address is contained within the selection.
	pub fn contains(&self, address: Address) -> bool {
		match *self {
			Select::Address(addr) => address == addr,
			Select::Line {page, line} 
				=> address.page == page && address.line == line,
			Select::Page {page} => address.page == page,
			Select::All => true,
		}
	}
}


impl Into<Address> for Select {
	fn into(self) -> Address {
		self.base_address()
	}
}

impl<'a> Into<Address> for &'a Select {
	fn into(self) -> Address {
		self.base_address()
	}
}


impl fmt::Display for Select {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match *self {
			Select::Address(addr) => write!(f, "{}", addr),
			Select::Line {page, line} => write!(f, "{}:{}:*", page, line),
			Select::Page {page} => write!(f, "{}:*:*", page),
			Select::All => write!(f, "*:*:*", ),
		}
	}
}