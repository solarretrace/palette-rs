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
//! Defines a metadata object for tracking information about palette data.
//!
////////////////////////////////////////////////////////////////////////////////
use address::{LineCount, ColumnCount};

use std::fmt;
use std::result;

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
	pub line_count_override: LineCount,
	/// An override to the default column count for this group.
	pub column_count_override: ColumnCount,
	/// Identifies whether the format's preparatory functions have been called 
	/// for this item already.
	pub initialized: bool,
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
			self.line_count_override, 
			self.column_count_override)
	}
}