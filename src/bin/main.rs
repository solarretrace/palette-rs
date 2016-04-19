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


fn main() {
	// let colors = vec![
	// 	("Black", 0x000000),
	// 	("White", 0xFFFFFF),
	// 	("Red", 0xFF0000),
	// 	("Lime", 0x00FF00),
	// 	("Blue", 0x0000FF),
	// 	("Yellow", 0xFFFF00),
	// 	("Cyan", 0x00FFFF),
	// 	("Magenta", 0xFF00FF),
	// 	("Silver", 0xC0C0C0),
	// 	("Gray", 0x808080),
	// 	("Maroon", 0x800000),
	// 	("Olive", 0x808000),
	// 	("Green", 0x008000),
	// 	("Purple", 0x800080),
	// 	("Teal", 0x008080),
	// 	("Navy", 0x000080),
	// ];

	// for (name, color) in colors {
	// 	let rgb = Rgb::from(color);
	// 	let hsl = Hsl::from(rgb);
	// 	let hsv = Hsv::from(rgb);
	// 	let cmyk = Cmyk::from(rgb);
	// 	let xyz = Xyz::from(rgb);
	// 	println!("{}\t{:X}\t{:?}\
	// 		\n\t{:X}\t{:?}\
	// 		\n\t{:X}\t{:?}\
	// 		\n\t{:X}\t{:?}\
	// 		\n\t{:X}\t{:?}\
	// 		\n", 
	// 		name,
	// 		rgb, rgb,
	// 		Rgb::from(cmyk), cmyk,
	// 		Rgb::from(hsl), hsl,
	// 		Rgb::from(hsv), hsv,
	// 		Rgb::from(xyz), xyz);
	// }

	// let mut c = Color::new(0x89, 0x66, 0x00);
	// println!("{:?}", c.hsv_components());
	// for _ in 0..16 {
	// 	// c.shift_hue(30.0);
	// 	c.darken(0.1);
	// 	println!("{:?}", c.hsv_components());
	// }


	let mut pal = Palette::new("Test Palette", Format::Default, true);
	
	pal.apply(Box::new(
		InsertColor::new(Rgb::from(0x404088))
			.located_at(Address::new(0, 0, 0))
	)).ok();

	pal.apply(Box::new(
		InsertColor::new(Rgb::from(0x00CC00))
			.located_at(Address::new(0, 0, 1))
	)).ok();

	pal.apply(Box::new(
		InsertRamp::new(Address::new(0, 0, 0), Address::new(0, 0, 1), 6)
			.located_at(Address::new(0, 1, 0))
	)).ok();

	println!("{}", pal);

	pal.apply(Box::new(
		InsertColor::new(Rgb::from(0xFFFFFF))
			.located_at(Address::new(0, 0, 1))
			.overwrite(true)
	)).ok();

	println!("{}", pal);
}
