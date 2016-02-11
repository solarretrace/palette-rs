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

extern crate rampeditor;

use rampeditor::Color;
use rampeditor::PaletteBuilder;
use rampeditor::Address;
use rampeditor::palette;

fn main() {
	
	let mut pal = PaletteBuilder::new()
		.using_format(palette::ZPL_FORMAT)
		.named("Super Duper Palette")
		.create();
	
	let a1 = Address::new(1, 2, 3);
	let a2 = Address::new(0, 1, 1);
	pal.set_color(a1, Color(0, 0, 0)).ok().expect("add color to pal");
	pal.set_color(a2, Color(123, 32, 200)).ok().expect("add color to pal");
	pal.add_ramp_between(a1, a2, 3).ok().expect("add ramp to pal");
	println!("{}", pal);

	pal.set_color(a2, Color(255, 0, 255)).ok().expect("add color to pal");
	println!("{}", pal);

	pal.delete_slot(Address::new(0,0,2));
	println!("{}", pal);	

}
