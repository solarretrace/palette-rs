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
//! Provides Page:Line:Column addressing for the `Palette`'s `Cell`s.
//!
//! When a color-expression refers to another cell, we have to supply a
//! reference to the cell or cells that are being operated on. Every cell is
//! assigned a unique identifier called its `Address`, which is comprised of
//! three components: the Page, Line, and Column. These components are always
//! positive integers.
//!
//! There are several ways to reference cells, and each of these must be
//! supported by the expression grammar, though functions within the palette may
//! imbue additional requirements on the references. 
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use result::{
	Result,
	Error,
};

// Non-local imports.
use interval::Interval;

// Standard imports.
use std::fmt;
use std::u16;
use std::u8;
use std::ops::Add;


////////////////////////////////////////////////////////////////////////////////
// Address internal types and limits.
////////////////////////////////////////////////////////////////////////////////
/// The page of an `Address`.
pub type Page = u16;
/// The line of an `Address`.
pub type Line = u8;
/// The column of an `Address`.
pub type Column = u8;

/// The maximum page in an Address.
pub const PAGE_MAX: Page = u16::MAX;
/// The maximum line in an Address.
pub const LINE_MAX: Line = u8::MAX;
/// The maximum column in an Address.
pub const COLUMN_MAX: Column = u8::MAX;

/// A page offset from an `Address`.
pub type PageOffset = i32;
/// A line offset from an `Address`.
pub type LineOffset = i16;
/// A column offset from an `Address`.
pub type ColumnOffset = i16;



////////////////////////////////////////////////////////////////////////////////
// Offset
////////////////////////////////////////////////////////////////////////////////
/// Allows computation of relative `Reference`s.
trait Offset: Sized + Copy {
	type Base: Add;
	fn offset(self, base: &Self::Base) -> Result<Self::Base>;
	fn is_negative(self) -> bool;
}

// Implementation for Page.
impl Offset for i32 {
	/// The base type to compute offsets from.
	type Base = u16;

	/// Computes the offset from the given component.
	fn offset(self, base: &Self::Base) -> Result<Self::Base> {
		let result = self + *base as Self;
		if result < 0 || result > (PAGE_MAX as Self) {
			Err(Error::InvalidReferenceComponent)
		} else {
			Ok(result as Self::Base)
		}
	}

	/// Returns whether the offset is negative.
	fn is_negative(self) -> bool {
		self < 0
	}
}

// Implementation for Line & Column.
impl Offset for i16 {
	type Base = u8;

	fn offset(self, base: &Self::Base) -> Result<Self::Base> {
		let result = self + *base as Self;
		if result < 0 || result > (LINE_MAX as Self) {
			Err(Error::InvalidReferenceComponent)
		} else {
			Ok(result as Self::Base)
		}
	}

	fn is_negative(self) -> bool {
		self < 0
	}
}



////////////////////////////////////////////////////////////////////////////////
// Reference
////////////////////////////////////////////////////////////////////////////////
/// A reference to a set of `Cell`s the in the palette.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Reference {
	/// The pages being referenced.
	page: ReferenceComponent<Page, PageOffset>,
	
	/// The lines being referenced.
	line: ReferenceComponent<Line, LineOffset>,

	/// The columns being referenced.
	column: ReferenceComponent<Column, ColumnOffset>,
}


impl Reference {
	/// Returns a `Reference` to the entire palette.
	pub fn all() -> Reference {
		use self::ReferenceComponent::*;

		Reference {
			page: All,
			line: All,
			column: All,
		}
	}

	/// Returns a `Reference` to the page containing the given `Address`.
	pub fn page_of(addr: &Address) -> Reference {
		use self::ReferenceComponent::*;

		Reference {
			page: Index(addr.page),
			line: All,
			column: All,
		}
	}

	/// Returns a `Reference` to the line containing the given `Address`.
	pub fn line_of(addr: &Address) -> Reference {
		use self::ReferenceComponent::*;

		Reference {
			page: Index(addr.page),
			line: Index(addr.line),
			column: All,
		}
	}

	/// Returns the page being referenced.
	///
	/// # Errors
	///
	/// Returns an `UnresolvedReferenceComponent` error when the reference does
	/// not resolve to a single page.
	pub fn page(&self) -> Result<Page> {
		use self::ReferenceComponent::*;

		if let Reference {page: Index(page), ..} = *self {
			Ok(page)
		} else {
			Err(Error::UnresolvedReferenceComponent)
		}
	}

	/// Returns the line being referenced.
	///
	/// # Errors
	///
	/// Returns an `UnresolvedReferenceComponent` error when the reference does
	/// not resolve to a single line.
	pub fn line(&self) -> Result<Line> {
		use self::ReferenceComponent::*;

		if let Reference {line: Index(line), ..} = *self {
			Ok(line)
		} else {
			Err(Error::UnresolvedReferenceComponent)
		}
	} 

	/// Returns the column being referenced.
	///
	/// # Errors
	///
	/// Returns an `UnresolvedReferenceComponent` error when the reference does
	/// not resolve to a single column.
	pub fn column(&self) -> Result<Column> {
		use self::ReferenceComponent::*;

		if let Reference {column: Index(column), ..} = *self {
			Ok(column)
		} else {
			Err(Error::UnresolvedReferenceComponent)
		}
	} 
}


impl From<Address> for Reference {
	fn from(addr: Address) -> Self {
		use self::ReferenceComponent::*;

		Reference {
			page: Index(addr.page),
			line: Index(addr.line),
			column: Index(addr.column),
		}
	}
}


impl Default for Reference {
	fn default() -> Self {
		use self::ReferenceComponent::*;

		Reference {
			page: All,
			line: All,
			column: All,
		}
	}
}


impl fmt::Display for Reference {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}/{}/{}", self.page, self.line, self.column)
	}
}



////////////////////////////////////////////////////////////////////////////////
// ReferenceComponent
////////////////////////////////////////////////////////////////////////////////
/// A potentially indirect component of a `Reference`.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum ReferenceComponent<T, O> {
	Any,
	Index(T),
	Named(String),
	#[allow(dead_code)]
	Indirect(DirectReferenceComponent<T>, O),
	All,
}

impl<T, O> ReferenceComponent<T, O>
	where 
		O: Offset<Base=T>,
		T: Add
{
	/// Resolves index-relative components to their absolute positions, relative
	/// to the given base index. 
	///
	/// # Errors
	///
	/// Returns an `InvalidReferenceComponent` error when the offset would
	/// overflow or underflow the component boundaries.
	#[allow(dead_code)]
	pub fn resolve_index_indirection(&mut self, base: T) -> Result<()> {
		use self::ReferenceComponent::*;

		let mut resolved = None;
		if let Indirect(ref drc, ref o) = *self {
			resolved = match *drc {
				DirectReferenceComponent::Any
					=> Some(o.offset(&base)?),

				DirectReferenceComponent::Index(ref i)
					=> Some(o.offset(i)?),

				_	=> None,
			}
		}

		if let Some(res) = resolved {
			*self = Index(res);
		}
		Ok(())
	}
}

impl<T, O> From<DirectReferenceComponent<T>> for ReferenceComponent<T, O> {
	fn from(drc: DirectReferenceComponent<T>) -> Self {
		use self::DirectReferenceComponent::*;
		match drc {
			Any			=> ReferenceComponent::Any,
			Index(i)	=> ReferenceComponent::Index(i),
			Named(name)	=> ReferenceComponent::Named(name),
		}
	}
}


impl<T, O> fmt::Display for ReferenceComponent<T, O> 
	where
		T: fmt::Display,
		O: fmt::Display + Offset,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::ReferenceComponent::*;

		match *self {
			Any						=> write!(f, "_"),
			Index(ref i)			=> write!(f, "{}", i),
			Named(ref name)			=> write!(f, "{}", name),
			Indirect(ref r, ref o)	=> if o.is_negative() {
					write!(f, "{}{}", r, o)
				} else {
					write!(f, "{}+{}", r, o)
				},
			All						=> write!(f, "*"),
		}
	}
}

////////////////////////////////////////////////////////////////////////////////
// DirectReferenceComponent
////////////////////////////////////////////////////////////////////////////////
/// A direct component of a `Reference`.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum DirectReferenceComponent<T> {
	Any,
	Index(T),
	Named(String),
}

impl<T, O> From<ReferenceComponent<T, O>> for DirectReferenceComponent<T> {
	fn from(rc: ReferenceComponent<T, O>) -> Self {
		use self::DirectReferenceComponent::*;
		match rc {
			ReferenceComponent::Any			=> Any,
			ReferenceComponent::Index(i)	=> Index(i),
			ReferenceComponent::Named(name)	=> Named(name),
			_	=> panic!("invalid reference component conversion"),
		}
	}
}


impl<T> fmt::Display for DirectReferenceComponent<T> where T: fmt::Display {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::DirectReferenceComponent::*;

		match *self {
			Any				=> write!(f, "_"),
			Index(ref i)	=> write!(f, "{}", i),
			Named(ref name)	=> write!(f, "{}", name),
		}
	}
}

////////////////////////////////////////////////////////////////////////////////
// Address
////////////////////////////////////////////////////////////////////////////////
/// The absolute position of a Cell.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone, Copy, Default)]
pub struct Address {
	/// The page of the Address.
	pub page: Page,

	/// The line of the Address.
	pub line: Line,
	
	/// The column of the Address.
	pub column: Column,
}


impl Address {
	/// Creates a new `Address`.
	pub fn new(page: Page, line: Line, column: Column) -> Self {
		Address {
			page: page,
			line: line,
			column: column,
		}
	}

	/// Returns the `Address` n steps ahead, assuming the given wrapping 
	/// parameters.
	///
	/// # Example
	///
	/// ```rust
	/// use palette::Address;
	/// 
	/// let a = Address::new(0, 9, 9);
	/// let b = a.wrapping_step(1, 10, 10, 10);
	/// 
	/// assert_eq!(b, Address::new(1, 0, 0));
	///
	/// let c = Address::new(0, 0, 0).wrapping_step(200, 5, 5, 5);
	/// assert_eq!(c, Address::new(3, 0, 0));
	/// ```
	pub fn wrapping_step(
		&self, 
		n: usize,
		pages: Page,
		lines: Line, 
		columns: Column) 
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
			d as Page % pages,
			(m / c) as Line,
			(m % c) as Column
		)
	}
}


impl Into<Selection> for Address {
	fn into(self) -> Selection {
		Selection::new([Interval::closed(self.clone(), self)].iter().cloned())
	}
}


impl fmt::Display for Address {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}:{}:{}", self.page, self.line, self.column)
	}
}


impl fmt::UpperHex for Address {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:02X}:{:02X}:{:02X}", self.page, self.line, self.column)
	}
}


impl fmt::LowerHex for Address {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:02x}:{:02x}:{:02x}", self.page, self.line, self.column)
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
	pub fn new<I>(intervals: I) -> Self 
		where I: IntoIterator<Item=Interval<Address>> 
	{
		Selection {
			inner: Interval::union_all(intervals.into_iter())
		}
	}

	/// Unions an interval into the selection.
	pub fn union(&mut self, interval: Interval<Address>) {
		self.inner.push(interval);
	}

	/// Returns whether the given address is contained in the selection.
	pub fn contains(&self, address: &Address) -> bool {
		self.inner.iter().any(|int| int.contains(address))
	}
}