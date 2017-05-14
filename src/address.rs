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
use interval::Interval;

use std::fmt;
use std::u16;
use std::u8;


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

	/// Returns the address n steps ahead, assuming the given wrapping 
	/// parameters.
	///
	/// # Example
	///
	/// ```rust
	/// use rampeditor::Address;
	/// 
	/// let a = Address::new(0, 9, 9);
	/// let b = a.wrapping_add(1, 10, 10, 10);
	/// 
	/// assert_eq!(b, Address::new(1, 0, 0));
	///
	/// let c = Address::new(0, 0, 0).wrapping_add(200, 5, 5, 5);
	/// assert_eq!(c, Address::new(3, 0, 0));
	/// ```
	pub fn wrapping_add(
		&self, 
		n: usize,
		pages: PageCount,
		lines: LineCount, 
		columns: ColumnCount) 
		-> Address
	{
		let (l, c) = (lines as usize, columns as usize);
		let n2 = n 
			+ self.page as usize * l * c
			+ self.line as usize * c
			+ self.column as usize;
		let d = n2 / (l * c);
		let m = n2 % (l * c);
		Address::new(
			d as PageCount % pages,
			(m / c) as LineCount,
			(m % c) as ColumnCount
		)
	}

	/// Returns the page group containing the address.
	///
	/// # Example
	///
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
	///
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
		Selection::new([Interval::closed(self.clone(), self)].iter().cloned())
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
		Selection::new(Some(match self {
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
				Address::new(PAGE_MAX, LINE_MAX, COLUMN_MAX)
			),
		}))
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



////////////////////////////////////////////////////////////////////////////////
// Selection
////////////////////////////////////////////////////////////////////////////////
/// A possibly non-contiguous selection of addresses.
#[derive(Default, Clone)]
pub struct Selection {
	inner: Vec<Interval<Address>>
}


impl Selection {
	/// Creates a new selection from a collection of address intervals.
	///
	/// # Example
	///
	/// ```rust
	/// use rampeditor::{Selection, Interval, Address};
	/// 
	/// let sel = Selection::new([
	///		Interval::open(Address::new(0, 0, 0), Address::new(1, 0, 0)),
	///		Interval::open(Address::new(3, 0, 0), Address::new(3, 5, 0)),
	///		Interval::closed(Address::new(8, 0, 6), Address::new(8, 0, 6))
	///	].iter().cloned());
	/// 
	/// assert!(sel.contains(&Address::new(8, 0, 6)));
	/// assert!(sel.contains(&Address::new(3, 0, 6)));
	/// assert!(!sel.contains(&Address::new(0, 0, 0)));
	/// ```
	pub fn new<I>(intervals: I) -> Self 
		where I: IntoIterator<Item=Interval<Address>> 
	{
		Selection {
			inner: Interval::union_all(intervals.into_iter())
		}
	}

	/// Unions an interval into the selection.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::{Selection, Interval, Address};
	/// 
	/// let mut sel = Selection::new([
	///		Interval::closed(Address::new(0, 0, 0), Address::new(1, 0, 0)),
	///	].iter().cloned());
	/// 
	/// assert!(!sel.contains(&Address::new(2, 0, 0)));
	///
	/// sel.union(Interval::open(Address::new(1, 0, 0), Address::new(4, 0, 0)));
	///
	/// assert!(sel.contains(&Address::new(2, 0, 0)));
	/// ```
	pub fn union(&mut self, interval: Interval<Address>) {
		self.inner.push(interval);
	}

	/// Returns whether the given address is contained in the selection.
	///
	/// # Example
	///
	/// ```rust
	/// # use rampeditor::{Selection, Interval, Address};
	/// 
	/// let sel = Selection::new([
	///		Interval::closed(Address::new(0, 0, 0), Address::new(1, 0, 0)),
	///	].iter().cloned());
	///
	/// assert!(sel.contains(&Address::new(0, 0, 1)));
	/// assert!(!sel.contains(&Address::new(10, 0, 0)));
	/// ```
	pub fn contains(&self, address: &Address) -> bool {
		self.inner.iter().any(|int| int.contains(address))
	}
}
