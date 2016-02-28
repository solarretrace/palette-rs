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
use palette::data::PaletteData;
use palette::format::Palette;
use palette::operations::PaletteOperation;
use palette::history::OperationHistory;
use palette;
use address::{Group, PageCount, LineCount, ColumnCount};

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
	core: PaletteData,
	history: OperationHistory,
}

impl ZplPalette {
	#[allow(unused_variables)]
	fn prepare_new_page(data: &mut PaletteData, group: Group) {
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

	#[allow(unused_variables)]
	fn prepare_new_line(data: &mut PaletteData, group: Group) {
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
		let mut pal = ZplPalette {
			core: Default::default(),
			history: OperationHistory::new(),
		};
		pal.core.set_label(Group::All, "ZplPalette 1.0.0");
		pal.core.set_name(Group::All, name.into());
		pal.core.page_count = ZPL_PAGE_LIMIT;
		pal.core.default_line_count = ZPL_DEFAULT_LINE_LIMIT;
		pal.core.default_column_count = ZPL_DEFAULT_COLUMN_LIMIT;
		pal.core.prepare_new_page = ZplPalette::prepare_new_page;
		pal.core.prepare_new_line = ZplPalette::prepare_new_line;
		pal
	}

	fn apply<O>(&mut self, operation: O)  -> palette::Result<()> 
		where O: PaletteOperation 
	{
		let entry = try!(operation.apply(&mut self.core));
		self.history.push(entry);
		Ok(())
	}

	fn write_palette<W>(&self, out_buf: &mut W) -> io::Result<()> 
		where W: io::Write
	{
		// Write header.
		try!(out_buf.write(&ZPL_HEADER)); 

		// let mut addresses = self.core.addresses();
		// let mut next = addresses.next();
		// // Write all pages in sequence.
		// for page in 0..ZPL_PAGE_LIMIT {
		// }

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

	#[allow(unused_variables)]
	fn read_palette<R>(in_buf: &R) -> io::Result<Self>
		where R: io::Read, Self: Sized
	{
		unimplemented!()
	}
}

impl fmt::Display for ZplPalette {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		write!(f, "{} {}\n\n{:#?}",
			self.core.get_label(Group::All).unwrap_or(""),
			self.core,
			self.history
		)
	}
}
