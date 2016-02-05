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

use rampeditor::core::color::Color;
use rampeditor::core::palette::PaletteBuilder;
use rampeditor::core::address::Address;

fn main() {
	
	let c1 = Color(34, 68, 122);
	let c2 = Color(188, 35, 123);
	let c3 = Color(0, 0, 0);
	let mut pal = PaletteBuilder::new()
		.with_column_wrap(6)
		.build();


	let a0 = pal.add_color(c1).ok().expect("Add color failed.");
	let a1 = pal.add_color(c2).ok().expect("Add color failed.");
	let a2 = pal.add_color(c3).ok().expect("Add color failed.");
	pal.add_ramp_between(a0, a2, 6).expect("Add ramp failed.");
	pal.add_ramp_between(a1, Address::new(0,0,5), 6).expect("Add ramp failed.");
	
	println!("{}", &pal);


}
