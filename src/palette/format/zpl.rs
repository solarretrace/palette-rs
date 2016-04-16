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
//! Provides components for interacting with the ZPL palette format.
//!
////////////////////////////////////////////////////////////////////////////////
use palette::Palette;
use palette::data::PaletteOperationData;
use palette::operation::PaletteOperation;
use palette;
use address::{Address, Group, PageCount, LineCount, ColumnCount};
use color::Rgb;

use std::fmt;
use std::result;

use std::io;
use std::io::{Result, Write, Read};

// The ZPL format was built for version 2.50 build 24 of Zelda Classic, and may
// not work on versions 1.92 or older.

const ZPL_COLOR_DEPTH_SCALE: f32 = 0.25;

const ZPL_HEADER : [u8;12] = [
	0x43, 0x53, 0x45, 0x54, 
	0x04, 0x00, 0x01, 0x00, 
	0x9c, 0x0d, 0x05, 0x00
];

const ZPL_FOOTER_A : [u8;4] = [
	0x5a, 0x00, 0x00, 0x00
];

const ZPL_FOOTER_B : [u8;4] = [ // x 109
	0x00, 0x00, 0x00, 0x00
];

const ZPL_FOOTER_C : [u8;20] = [
	0x00, 0x00, 0x00, 0x36, 
	0x00, 0x00, 0x4e, 0x00, 
	0x00, 0x14, 0x00, 0x00, 
	0x36, 0x00, 0x00, 0x4e,
	0x00, 0x00, 0x14, 0x00
];

const ZPL_FOOTER_D : [u8;4] = [ // x 79
	0x00, 0x00, 0x00, 0x00
];

const ZPL_FOOTER_E : [u8;36] = [
	0x22, 0x00, 0x00, 0x66, 
	0x00, 0x00, 0x5a, 0x00, 
	0x00, 0x22, 0x00, 0x00, 
	0x86, 0x00, 0x00, 0x3c,
	0x00, 0x00, 0x22, 0x00, 
	0x00, 0x86, 0x00, 0x00, 
	0x3c, 0x00, 0x00, 0x20, 
	0x30, 0x40, 0x3f, 0x3f, 
	0x3f, 0x07, 0x07, 0x07
];

const ZPL_PAGE_LIMIT: PageCount =  0x203;
const ZPL_DEFAULT_LINE_LIMIT: LineCount =  16;
const ZPL_DEFAULT_COLUMN_LIMIT: ColumnCount =  16;

const MAIN_PAGE_LIMIT: PageCount = 0;
const LEVEL_PAGE_LIMIT: PageCount = 512;
const SPRITE_PAGE_LIMIT: PageCount = 515;

////////////////////////////////////////////////////////////////////////////////
// ZplPalette
////////////////////////////////////////////////////////////////////////////////
/// The default palette format with no special configuration.
#[derive(Debug)]
pub struct ZplPalette {
	inner: PaletteOperationDataWithHistory,
}

impl ZplPalette {
	fn prepare_new_page(data: &mut PaletteOperationData, group: Group) {
		if let Group::Page {page} = group {
			if page <= MAIN_PAGE_LIMIT {
				data.set_name(group, "Main");
				data.set_label(group, "Level 0");
				data.set_line_count(group, 14);
			} else if page <= LEVEL_PAGE_LIMIT {
				data.set_label(group, format!("Level {}", page));
			} else {
				data.set_label(group, format!("Sprite Page {}", page));
			}
		}
	}

	fn prepare_new_line(data: &mut PaletteOperationData, group: Group) {
		if let Group::Line {page, line} = group {
			if page <= MAIN_PAGE_LIMIT {
				data.set_label(group, format!("Main CSET {}", line));
			} else if page <= LEVEL_PAGE_LIMIT {
				data.set_label(group, 
					ZplPalette::get_level_label_for_line(line)
				);
			} else {
				data.set_label(group, 
					format!("Sprite CSET {}", page as usize 
						- LEVEL_PAGE_LIMIT as usize + line as usize
					)
				);
			}
		}
	}

	fn get_level_label_for_line(line: LineCount) -> String {
		format!("CSET {} ({})", line,
			if line == 0 || line == 4 || line == 7 || line == 10 {
				"2"
			} else if line == 1 || line == 5 || line == 8 || line == 11 {
				"3"
			} else if line == 2 || line == 6 || line == 9 || line == 12 {
				"4"
			} else {
				"9"
			}
		)
	}
}

impl Palette for ZplPalette {
	fn new<S>(name: S) -> Self where S: Into<String> {
		let mut inner: PaletteOperationDataWithHistory = Default::default();
		
		inner.set_label(Group::All, "ZplPalette 1.0.0");
		inner.set_name(Group::All, name.into());
		inner.page_count = ZPL_PAGE_LIMIT;
		inner.default_line_count = ZPL_DEFAULT_LINE_LIMIT;
		inner.default_column_count = ZPL_DEFAULT_COLUMN_LIMIT;
		inner.prepare_new_page = ZplPalette::prepare_new_page;
		inner.prepare_new_line = ZplPalette::prepare_new_line;
		
		ZplPalette {
			inner: inner,
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


	/// Reverses the most recently applied operation.
	fn undo(&mut self) -> palette::Result<()> {
		let mut inner = &mut self.inner;
		if inner.operation_history.is_some() {
			let mut h = inner.operation_history.as_ref().unwrap().borrow_mut();

			if let Some(mut entry) = h.undo_entries.pop() {
				let redo = try!(entry.undo.apply(inner));
				h.redo_entries.push(redo);
			}
			Ok(())

		} else {
			panic!("undo not supported")
		}

		// if let Some(ref history) = self.inner.operation_history {
		// 	let mut h = history.borrow_mut();

		// 	if let Some(mut entry) = h.undo_entries.pop() {
		// 		let redo = try!(entry.undo.apply(&mut self.inner));
		// 		h.redo_entries.push(redo);
		// 	}
		// 	Ok(())
		// } else {
		// 	panic!("undo not supported")
		// }
	}

	/// Reverses the most recently applied undo operation.
	fn redo(&mut self) -> palette::Result<()> {
		if let Some(ref history) = self.inner.operation_history {
			let h = history.borrow_mut();
			let (ref u, ref r) = (h.undo_entries, h.redo_entries);
			if let Some(mut entry) = r.pop() {
				let undo = try!(entry.undo.apply(&mut self.inner));
				u.push(undo);
			}
			Ok(())
		} else {
			panic!("undo not supported")
		}
	}

	fn write_palette<W>(&self, out_buf: &mut W) -> io::Result<()> 
		where W: io::Write
	{
		// Write header.
		try!(out_buf.write(&ZPL_HEADER));

		// Write all pages in sequence.

		// Write level names.

		// Write footer.
		try!(out_buf.write(&ZPL_FOOTER_A));
		for _ in 1..109 {
			try!(out_buf.write(&ZPL_FOOTER_B));
		}
		try!(out_buf.write(&ZPL_FOOTER_C));
		for _ in 1..79 {
			try!(out_buf.write(&ZPL_FOOTER_D));
		}
		try!(out_buf.write(&ZPL_FOOTER_E));
		Ok(())
	}
}

impl fmt::Display for ZplPalette {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		write!(f, "{} [History: {} items]\n{}",
			self.inner.get_label(Group::All).unwrap_or(""),
			self.inner.history_len(),
			self.inner,
		)
	}
}
