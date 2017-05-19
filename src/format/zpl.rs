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
//! The ZPL format was built for version 2.50 build 24 of Zelda Classic, and may
//! not work on versions 1.92 or older.
//!
////////////////////////////////////////////////////////////////////////////////

use address::{
	Reference,
	Page, Line, Column};
use data::Data;


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

const ZPL_PAGE_LIMIT: Page =  0x203;
const ZPL_DEFAULT_LINE_LIMIT: Line =  16;
const ZPL_DEFAULT_COLUMN_LIMIT: Column =  16;

const MAIN_PAGE_LIMIT: Page = 0;
const LEVEL_PAGE_LIMIT: Page = 512;
const SPRITE_PAGE_LIMIT: Page = 515;


/// Returns the level label for the given line.
fn get_level_label(line: Line) -> String {
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


/// Called when a new palette is created. Initializes the palette data.
pub fn initialize(data: &mut Data) {
	data.set_label(Reference::all(), "ZplPalette 1.0.0");
	data.maximum_page_count = ZPL_PAGE_LIMIT;
	data.default_line_count = ZPL_DEFAULT_LINE_LIMIT;
	data.default_column_count = ZPL_DEFAULT_COLUMN_LIMIT;
	data.prepare_new_page = prepare_new_page;
	data.prepare_new_line = prepare_new_line;
}

	
/// The function to call when a new page is created.
#[cfg_attr(feature = "cargo-clippy", allow(absurd_extreme_comparisons))]
pub fn prepare_new_page(data: &mut Data, group: &Reference) {
	let page = group.page().expect("prepare page group with page reference");
	if page <= MAIN_PAGE_LIMIT {
		data.set_name(group.clone(), "Main");
		data.set_label(group.clone(), "Level 0");
		data.set_line_count(group.clone(), 14);
	} else if page <= LEVEL_PAGE_LIMIT {
		data.set_label(group.clone(), format!("Level {}", page));
	} else {
		data.set_label(group.clone(), format!("Sprite Page {}", page));
	}
}


/// The function to call when a new line is created.
#[cfg_attr(feature = "cargo-clippy", allow(absurd_extreme_comparisons))]
pub fn prepare_new_line(data: &mut Data, group: &Reference) {
	let page = group.page().expect("prepare line group with page reference");
	let line = group.line().expect("prepare line group with line reference");

	if page <= MAIN_PAGE_LIMIT {
		data.set_label(group.clone(), format!("Main CSET {}", line));
	} else if page <= LEVEL_PAGE_LIMIT {
		data.set_label(group.clone(), 
			get_level_label(line)
		);
	} else {
		data.set_label(group.clone(), 
			format!("Sprite CSET {}", page as usize 
				- LEVEL_PAGE_LIMIT as usize + line as usize
			)
		);
	}
	
}



	// fn write_palette<W>(&self, out_buf: &mut W) -> io::Result<()> 
	// 	where W: io::Write
	// {
	// 	// Write header.
	// 	out_buf.write(&ZPL_HEADER)?;

	// 	// Write all pages in sequence.

	// 	// Write level names.

	// 	// Write footer.
	// 	out_buf.write(&ZPL_FOOTER_A)?;
	// 	for _ in 1..109 {
	// 		out_buf.write(&ZPL_FOOTER_B)?;
	// 	}
	// 	out_buf.write(&ZPL_FOOTER_C)?;
	// 	for _ in 1..79 {
	// 		out_buf.write(&ZPL_FOOTER_D)?;
	// 	}
	// 	out_buf.write(&ZPL_FOOTER_E)?;
	// 	Ok(())
	// }

