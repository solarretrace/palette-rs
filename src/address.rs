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
//! Provides a Page:Line:Column addressing object for organizing the palette.
//!
////////////////////////////////////////////////////////////////////////////////
use super::Interval;

use std::fmt;
use std::u16;
use std::u8;
use std::collections::HashSet;


////////////////////////////////////////////////////////////////////////////////
// Address internal types and limits.
////////////////////////////////////////////////////////////////////////////////
/// A type alias for the pages of an Address.
pub type PageCount = u16;
/// A type alias for the lines of an Address.
pub type LineCount = u8;
/// A type alias for the columns of an Address.
pub type ColumnCount = u8;

/// The maximum page in an Address.
pub const PAGE_MAX: PageCount = u16::MAX;
/// The maximum line in an Address.
pub const LINE_MAX: LineCount = u8::MAX;
/// The maximum column in an Address.
pub const COLUMN_MAX: ColumnCount = u8::MAX;


////////////////////////////////////////////////////////////////////////////////
// Selection
////////////////////////////////////////////////////////////////////////////////
/// A set of address intervals.
pub type Selection = HashSet<Interval<Address>>;


////////////////////////////////////////////////////////////////////////////////
// Address
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single address name.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone, Copy, Default)]
pub struct Address {
	/// The page of the Address.
	pub page: PageCount,
	/// The line of the Address.
	pub line: LineCount,
	/// The column of the Address.
	pub column: ColumnCount,
}


impl Address {
	/// Creates a new Address.
	pub fn new(page: PageCount, line: LineCount, column: ColumnCount) -> Self {
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
	/// use rampeditor::Address;
	/// 
	/// let a = Address::new(0, 9, 9);
	/// let b = a.wrapped_next(10, 10, 10);
	/// 
	/// assert_eq!(b, Address::new(1, 0, 0));
	/// ```
	pub fn wrapped_next(
		&self, 
		pages: PageCount,
		lines: LineCount, 
		columns: ColumnCount) 
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

	/// Returns the page group containing the address.
	///
	/// # Example
	/// ```rust
	/// use rampeditor::{Address, Group};
	/// 
	/// let a = Address::new(1, 2, 3).page_group();
	/// 
	/// assert_eq!(a, Group::Page {page: 1});
	/// ```
	pub fn page_group(&self) -> Group {
		Group::Page {page: self.page}
	}

	/// Returns the line group containing the address.
	///
	/// # Example
	/// ```rust
	/// use rampeditor::{Address, Group};
	/// 
	/// let a = Address::new(1, 2, 3).line_group();
	/// 
	/// assert_eq!(a, Group::Line {page: 1, line: 2});
	/// ```
	pub fn line_group(&self) -> Group {
		Group::Line {page: self.page, line: self.line}
	}
}

impl Into<Selection> for Address {
	fn into(self) -> Selection {
		[Interval::closed(self.clone(), self)].iter().cloned().collect()
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


////////////////////////////////////////////////////////////////////////////////
// Group
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates the group of a single line, page, or palette.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone, Copy)]
pub enum Group {
	/// A single line group.
	Line {
		/// The page of the group.
		page: PageCount, 
		/// The line of the group.
		line: LineCount
	},
	/// A single page group.
	Page {
		/// The page of the group.
		page: PageCount
	},
	/// A full palette group.
	All,
}


impl Group {
	/// Returns the first address located within the group.
	pub fn base_address(&self) -> Address {
		match *self {
			Group::Line {page, line} => Address::new(page, line, 0),
			Group::Page {page} => Address::new(page, 0, 0),
			Group::All => Address::new(0, 0, 0),
		}
	}

	/// Returns whether the address is contained within the group.
	pub fn contains(&self, address: Address) -> bool {
		match *self {
			Group::Line {page, line} 
				=> address.page == page && address.line == line,
			Group::Page {page} => address.page == page,
			Group::All => true,
		}
	}
}


impl Into<Selection> for Group {
	fn into(self) -> Selection {
		[
			match self {
				Group::Line {page, line} => Interval::right_open(
					Address::new(page, line, 0),
					Address::new(page, line+1, 0)
				),
				Group::Page {page} => Interval::right_open(
					Address::new(page, 0, 0),
					Address::new(page+1, 0, 0)
				),
				Group::All => Interval::closed(
					Address::new(0, 0, 0),
					Address::new(
						PAGE_MAX, 
						LINE_MAX, 
						COLUMN_MAX
					),
				),
			}
		].iter().cloned().collect()
	}
}


impl fmt::Display for Group {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match *self {
			Group::Line {page, line} => write!(f, "{}:{}:*", page, line),
			Group::Page {page} => write!(f, "{}:*:*", page),
			Group::All => write!(f, "*:*:*", ),
		}
	}
}