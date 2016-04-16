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
//! Provides components for interacting with the default palette format.
//!
////////////////////////////////////////////////////////////////////////////////
use palette::Palette;
use palette::data::PaletteOperationData;
use palette::operation::PaletteOperation;
use palette;
use address::{Address, Group};
use color::Rgb;

use std::fmt;
use std::result;

////////////////////////////////////////////////////////////////////////////////
// BasicPalette
////////////////////////////////////////////////////////////////////////////////
/// A basic palette format with no special configuration or functionality.
#[derive(Debug)]
pub struct BasicPalette {
	inner: PaletteOperationData,
}

impl Palette for BasicPalette {

	fn new<S>(name: S) -> Self where S: Into<String> {
		let mut inner: PaletteOperationData = Default::default();

		inner.set_label(Group::All, "BasicPalette 1.0.0");
		inner.set_name(Group::All, name.into());
		
		BasicPalette {
			inner: inner
		}
	}

	fn get_color(&self, address: Address) -> Option<Rgb> {
		self.inner.get_slot(address).and_then(|slot| slot.get_color())
	}

	fn len(&self) -> usize {
		self.inner.len()
	}

	fn apply_operation(&mut self, mut operation: Box<PaletteOperation>) 
		-> palette::Result<()> 
	{
		let entry = try!(operation.apply(&mut self.inner));

		if let Some(ref history) = self.inner.operation_history {
			history.borrow_mut().undo_entries.push(entry);
			history.borrow_mut().redo_entries.clear();
		}
		Ok(())
	}


	// Reverses the most recently applied operation.
	// fn undo(&mut self) -> palette::Result<()> {
	// 	if let Some((ref mut u, ref mut r)) = self.inner.operation_history {
	// 		if let Some(mut entry) = u.pop() {
	// 			let redo = try!(entry.undo.apply(&mut self.inner));
	// 			r.push(redo);
	// 		}
	// 		Ok(())
	// 	} else {
	// 		panic!("undo not supported")
	// 	}
	// }

	// Reverses the most recently applied undo operation.
	// fn redo(&mut self) -> palette::Result<()> {
	// 	if let Some((ref mut u, ref mut r)) = self.inner.operation_history {
	// 		if let Some(mut entry) = r.pop() {
	// 			let undo = try!(entry.undo.apply(&mut self.inner));
	// 			u.push(undo);
	// 		}
	// 		Ok(())
	// 	} else {
	// 		panic!("undo not supported")
	// 	}
	// }
}

impl fmt::Display for BasicPalette {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		write!(f, "{} {}",
			self.inner.get_label(Group::All).unwrap_or(""),
			self.inner
		)
	}
}
