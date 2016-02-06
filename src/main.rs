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

fn main() {
	
	let c1 = Color(34, 68, 122);
	let c2 = Color(188, 35, 123);
	let c3 = Color(0, 0, 0);
	let mut pal = PaletteBuilder::new()
		.with_column_count(6)
		.build();


	let a0 = pal.add_color(c1).ok().expect("add color");
	let a1 = pal.add_color(c2).ok().expect("add color");
	let a2 = pal.add_color(c3).ok().expect("add color");
	pal.add_ramp_between(a0, a2, 2).expect("add ramp");
	pal.add_ramp_between(a1, Address::new(0,0,3), 2).expect("add ramp");

	println!("{}", &pal);

	pal.set_color(a2, Color(45, 45, 0)).ok();

	println!("{}", &pal);


}
