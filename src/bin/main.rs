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

use rampeditor::*;
// use std::fs::File;

fn main() {
	let mut pal = ZplPalette::new("Test Palette");
	
	pal.apply(
		InsertColor::new(Color(50, 50, 78))
			.located_at(Address::new(0, 0, 0))
	).ok();

	pal.apply(
		InsertColor::new(Color(0, 0, 255))
			.located_at(Address::new(0, 0, 1))
	).ok();

	pal.apply(
		InsertRamp::new(Address::new(0, 0, 0), Address::new(0, 0, 1), 6)
			.located_at(Address::new(0, 1, 0))
	).ok();

	println!("{}", pal);

	pal.apply(
		InsertColor::new(Color(0, 100, 100))
			.located_at(Address::new(0, 0, 0))
			.overwrite(true)
	).ok();

	println!("{}", pal);
}
