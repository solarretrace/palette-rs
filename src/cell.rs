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
//! Provides `Cell`, which encapsulates the a location in a `Palette` that will
//! resolve to a single `Color`.
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use expression::Expression;

// Non-local imports.
use color::Color;

// Standard imports.
use std::cell::RefCell;
use std::ops::{
	Deref,
	DerefMut,
};



////////////////////////////////////////////////////////////////////////////////
// Cell
////////////////////////////////////////////////////////////////////////////////
/// A wrapper around a `Expression` for enabling interior mutability.
#[derive(Debug, Clone)]
pub struct Cell {
	/// The `Expression` being wrapped.
	expr: RefCell<Expression>,
}


impl Cell {
	/// Creates a new `Cell` wrapping the given `Expression`.
	pub fn new(element: Expression) -> Self {
		Cell {
			expr: RefCell::new(element),
		}
	}

	/// Returns the `Color` of the internal `Expression`, or `None` if it is 
	/// invalid.
	pub fn color(&self) -> Option<Color> {
		self.expr.borrow().color()
	}
}


impl Deref for Cell {
	type Target = RefCell<Expression>;
	fn deref(&self) -> &Self::Target {
		&self.expr
	}
}


impl DerefMut for Cell {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.expr
	}
}

