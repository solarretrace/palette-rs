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
//! Defines the color space that the Palette is compatable with.
//!
////////////////////////////////////////////////////////////////////////////////


#[warn(missing_docs)]
pub mod cmyk;

#[warn(missing_docs)]
pub mod hsl;

#[warn(missing_docs)]
pub mod rgb;

pub use color::cmyk::*;
pub use color::hsl::*;
pub use color::rgb::*;


use std::fmt;

/// Standard SRGB gamma correction matrix. This gives the relative intensities 
/// of each RGB color component.
#[allow(dead_code)]
const SRGB_GAMMA_CORRECTION: [[f32; 3]; 3] = [
	[0.2125, 0.0,	  0.0	],
	[0.0,	  0.7154, 0.0	],
	[0.0,	  0.0,	  0.0721]
];


////////////////////////////////////////////////////////////////////////////////
// Color
////////////////////////////////////////////////////////////////////////////////
/// An RGB encoded color with extension methods.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone, Copy, Default)]
pub struct Color {
	/// The base RGB format of the color.
	pub rgb: Rgb
}

impl Color {
	/// Creates a new Color from RGB components.
	pub fn new(red: u8, green: u8, blue: u8) -> Self {
		Color {
			rgb: Rgb {r: red, g: green, b: blue}
		}
	}

	/// Returns the red component.
	pub fn red(&self) -> u8 {
		self.rgb.r
	}
	
	/// Returns the green component.
	pub fn green(&self) -> u8 {
		self.rgb.g
	}
	
	/// Returns the blue component.
	pub fn blue(&self) -> u8 {
		self.rgb.b
	}

	/// Returns the cyan component.
	pub fn cyan(&self) -> u8 {
		Cmyk::from(self.rgb).c
	}

	/// Returns the magenta component.
	pub fn magenta(&self) -> u8 {
		Cmyk::from(self.rgb).m
	}

	/// Returns the yellow component.
	pub fn yellow(&self) -> u8 {
		Cmyk::from(self.rgb).y
	}

	/// Returns the key component.
	pub fn key(&self) -> u8 {
		Cmyk::from(self.rgb).k
	}

	/// Returns the hue.
	pub fn hue(&self) -> f32 {
		Hsl::from(self.rgb).hue()
	}

	/// Returns the saturation.
	pub fn saturation(&self) -> f32 {
		Hsl::from(self.rgb).saturation()
	}

	/// Returns the lightness.
	pub fn lightness(&self) -> f32 {
		Hsl::from(self.rgb).lightness()
	}
	
	/// Sets the red component.
	pub fn set_red(&mut self, value: u8) {
		self.rgb.r = value;
	}
	
	/// Sets the green component.
	pub fn set_green(&mut self, value: u8) {
		self.rgb.g = value;
	}

	/// Sets the blue component.
	pub fn set_blue(&mut self, value: u8) {
		self.rgb.b = value;
	}

	/// Sets the cyan component.
	pub fn set_cyan(&mut self, value: u8) {
		let mut t = Cmyk::from(self.rgb);
		t.c = value;
		self.rgb = Rgb::from(t);
	}

	/// Sets the magenta component.
	pub fn set_magenta(&mut self, value: u8) {
		let mut t = Cmyk::from(self.rgb);
		t.m = value;
		self.rgb = Rgb::from(t);
	}

	/// Sets the yellow component.
	pub fn set_yellow(&mut self, value: u8) {
		let mut t = Cmyk::from(self.rgb);
		t.y = value;
		self.rgb = Rgb::from(t);
	}

	/// Sets the key component.
	pub fn set_key(&mut self, value: u8) {
		let mut t = Cmyk::from(self.rgb);
		t.k = value;
		self.rgb = Rgb::from(t);
	}

	/// Sets the hue.
	pub fn set_hue(&mut self, value: f32) {
		let mut t = Hsl::from(self.rgb);
		t.set_hue(value);
		self.rgb = Rgb::from(t);
	}

	/// Sets the saturation.
	pub fn set_saturation(&mut self, value: f32) {
		let mut t = Hsl::from(self.rgb);
		t.set_saturation(value);
		self.rgb = Rgb::from(t);
	}

	/// Sets the lightness.
	pub fn set_lightness(&mut self, value: f32) {
		let mut t = Hsl::from(self.rgb);
		t.set_lightness(value);
		self.rgb = Rgb::from(t);
	}

	/// Returns an array containing the [R, G, B] component octets.
	pub fn rgb_octets(&self) -> [u8; 3] {
		self.rgb.octets()
	}

	/// Returns an array containing the [C, M, Y, K] component octets.
	pub fn cmyk_octets(&self) -> [u8; 4] {
		Cmyk::from(self.rgb).octets()
	}

	/// Returns an array containing the [H, S, L] components.
	pub fn hsl_components(&self) -> [f32; 3] {
		Hsl::from(self.rgb).components()
	}

	/// Returns an array containing the [R, G, B] component ratios.
	pub fn rgb_ratios(&self) -> [f32; 3] {
		self.rgb.ratios()
	}

	/// Returns an array containing the [C, M, Y, K] component ratios.
	pub fn cmyk_ratios(&self) -> [f32; 4] {
		Cmyk::from(self.rgb).ratios()
	}

	/// Returns the RGB hex code.
	pub fn rgb_hex(&self) -> u32 {
		self.rgb.hex()
	}

	/// Returns the CMYK hex code.
	pub fn cmyk_hex(&self) -> u32 {
		Cmyk::from(self.rgb).hex()
	}

	/// Performs an RGB component-wise linear interpolation between the colors 
	/// `start` and `end`, returning the color located at the ratio given by 
	/// `amount`, which is clamped between 1 and 0.
	pub fn rgb_lerp<C>(start: C, end: C, amount: f32) -> Self 
		where C: Into<Rgb> + Sized
	{
		Rgb::lerp(start.into(), end.into(), amount).into()
	}

	/// Performs a CMYK component-wise linear interpolation between the colors 
	/// `start` and `end`, returning the color located at the ratio given by 
	/// `amount`, which is clamped between 1 and 0.
	pub fn cmyk_lerp<C>(start: C, end: C, amount: f32) -> Self 
		where C: Into<Cmyk> + Sized
	{
		Cmyk::lerp(start.into(), end.into(), amount).into()
	}

	/// Performs an HSL component-wise linear interpolation between the colors 
	/// `start` and `end`, returning the color located at the ratio given by 
	/// `amount`, which is clamped between 1 and 0.
	pub fn hsl_lerp<C>(start: C, end: C, amount: f32) -> Self 
		where C: Into<Hsl> + Sized
	{
		Hsl::lerp(start.into(), end.into(), amount).into()
	}

	/// Returns the distance between the given colors in RGB color space.
	pub fn rgb_distance<C>(start: C, end: C) -> f32 
		where C: Into<Rgb> + Sized
	{
		Rgb::distance(start.into(), end.into())
	}

	/// Returns the distance between the given colors in CMYK color space.
	pub fn cmyk_distance<C>(start: C, end: C) -> f32 
		where C: Into<Cmyk> + Sized
	{
		Cmyk::distance(start.into(), end.into())
	}

	/// Returns the distance between the given colors in HSL color space.
	pub fn hsl_distance<C>(start: C, end: C) -> f32 
		where C: Into<Hsl> + Sized
	{
		Hsl::distance(start.into(), end.into())
	}
}



impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:?}", self)
	}
}


impl fmt::UpperHex for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "#{:X}", self.rgb)
	}
}


impl fmt::LowerHex for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "#{:x}", self.rgb)
	}
}


impl From<Cmyk> for Color {
	fn from(cmyk: Cmyk) -> Color {
		Color {rgb: Rgb::from(cmyk)}
	}
}

impl From<Hsl> for Color {
	fn from(hsl: Hsl) -> Color {
		Color {rgb: Rgb::from(hsl)}
	}
}

impl From<Rgb> for Color {
	fn from(rgb: Rgb) -> Color {
		Color {rgb: rgb}
	}
}




////////////////////////////////////////////////////////////////////////////////
// Test Module
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::{Cmyk, Hsl, Rgb};
    use super::super::utilities::nearly_equal;

    /// Tests color conversions.
    #[test]
    fn color_conversions() {

 		// Black conversions.
 		let black = Rgb::from(0x000000);
 		let black_hsl = Hsl::from(black);
 		let black_cmyk = Cmyk::from(black);
 		println!("Testing Rgb to Hsl. black = {:?}", black_hsl);
 		assert!(nearly_equal(black_hsl.hue(), 0f32));
 		assert!(nearly_equal(black_hsl.saturation() * 100f32, 0f32));
 		assert!(nearly_equal(black_hsl.lightness() * 100f32, 0f32));
 		println!("Testing Rgb to Cmyk. black = {:?}", black_cmyk);
 		assert_eq!(black_cmyk.c, 0);
 		assert_eq!(black_cmyk.m, 0);
 		assert_eq!(black_cmyk.y, 0);
 		assert_eq!(black_cmyk.k, 255);
 		let black_rgb_a = Rgb::from(black_hsl);
 		println!("Testing Hsl to Rgb. black = {:?}", black_rgb_a);
 		assert_eq!(black_rgb_a, black);
 		let black_rgb_b = Rgb::from(black_cmyk);
 		println!("Testing Cmyk to Rgb. black = {:?}", black_rgb_b);
 		assert_eq!(black_rgb_b, black);

 		// White conversions.
 		let white = Rgb::from(0xFFFFFF);
 		let white_hsl = Hsl::from(white);
 		let white_cmyk = Cmyk::from(white);
 		println!("Testing Rgb to Hsl. white = {:?}", white_hsl);
 		assert!(nearly_equal(white_hsl.hue(), 0f32));
 		assert!(nearly_equal(white_hsl.saturation() * 100f32, 0f32));
 		assert!(nearly_equal(white_hsl.lightness() * 100f32, 100f32));
 		println!("Testing Rgb to Cmyk. white = {:?}", white_cmyk);
 		assert_eq!(white_cmyk.c, 0);
 		assert_eq!(white_cmyk.m, 0);
 		assert_eq!(white_cmyk.y, 0);
 		assert_eq!(white_cmyk.k, 0);
 		let white_rgb_a = Rgb::from(white_hsl);
 		println!("Testing Hsl to Rgb. white = {:?}", white_rgb_a);
 		assert_eq!(white_rgb_a, white);
 		let white_rgb_b = Rgb::from(white_cmyk);
 		println!("Testing Cmyk to Rgb. white = {:?}", white_rgb_b);
 		assert_eq!(white_rgb_b, white);

 		// Red conversions.
 		let red = Rgb::from(0xFF0000);
 		let red_hsl = Hsl::from(red);
 		let red_cmyk = Cmyk::from(red);
 		println!("Testing Rgb to Hsl. red = {:?}", red_hsl);
 		assert!(nearly_equal(red_hsl.hue(), 0f32));
 		assert!(nearly_equal(red_hsl.saturation() * 100f32, 100f32));
 		assert!(nearly_equal(red_hsl.lightness() * 100f32, 50f32));
 		println!("Testing Rgb to Cmyk. red = {:?}", red_cmyk);
 		assert_eq!(red_cmyk.c, 0);
 		assert_eq!(red_cmyk.m, 255);
 		assert_eq!(red_cmyk.y, 255);
 		assert_eq!(red_cmyk.k, 0);
 		let red_rgb_a = Rgb::from(red_hsl);
 		println!("Testing Hsl to Rgb. red = {:?}", red_rgb_a);
 		assert_eq!(red_rgb_a, red);
 		let red_rgb_b = Rgb::from(red_cmyk);
 		println!("Testing Cmyk to Rgb. red = {:?}", red_rgb_b);
 		assert_eq!(red_rgb_b, red);

 		// Lime conversions.
 		let lime = Rgb::from(0x00FF00);
 		let lime_hsl = Hsl::from(lime);
 		let lime_cmyk = Cmyk::from(lime);
 		println!("Testing Rgb to Hsl. lime = {:?}", lime_hsl);
 		assert!(nearly_equal(lime_hsl.hue(), 120f32));
 		assert!(nearly_equal(lime_hsl.saturation() * 100f32, 100f32));
 		assert!(nearly_equal(lime_hsl.lightness() * 100f32, 50f32));
 		println!("Testing Rgb to Cmyk. lime = {:?}", lime_cmyk);
 		assert_eq!(lime_cmyk.c, 255);
 		assert_eq!(lime_cmyk.m, 0);
 		assert_eq!(lime_cmyk.y, 255);
 		assert_eq!(lime_cmyk.k, 0);
 		let lime_rgb_a = Rgb::from(lime_hsl);
 		println!("Testing Hsl to Rgb. lime = {:?}", lime_rgb_a);
 		assert_eq!(lime_rgb_a, lime);
 		let lime_rgb_b = Rgb::from(lime_cmyk);
 		println!("Testing Cmyk to Rgb. lime = {:?}", lime_rgb_b);
 		assert_eq!(lime_rgb_b, lime);

 		// Blue conversions.
 		let blue = Rgb::from(0x0000FF);
 		let blue_hsl = Hsl::from(blue);
 		let blue_cmyk = Cmyk::from(blue);
 		println!("Testing Rgb to Hsl. blue = {:?}", blue_hsl);
 		assert!(nearly_equal(blue_hsl.hue(), 240f32));
 		assert!(nearly_equal(blue_hsl.saturation() * 100f32, 100f32));
 		assert!(nearly_equal(blue_hsl.lightness() * 100f32, 50f32));
 		println!("Testing Rgb to Cmyk. blue = {:?}", blue_cmyk);
 		assert_eq!(blue_cmyk.c, 255);
 		assert_eq!(blue_cmyk.m, 255);
 		assert_eq!(blue_cmyk.y, 0);
 		assert_eq!(blue_cmyk.k, 0);
 		let blue_rgb_a = Rgb::from(blue_hsl);
 		println!("Testing Hsl to Rgb. blue = {:?}", blue_rgb_a);
 		assert_eq!(blue_rgb_a, blue);
 		let blue_rgb_b = Rgb::from(blue_cmyk);
 		println!("Testing Cmyk to Rgb. blue = {:?}", blue_rgb_b);
 		assert_eq!(blue_rgb_b, blue);

 		// Yellow conversions.
 		let yellow = Rgb::from(0xFFFF00);
 		let yellow_hsl = Hsl::from(yellow);
 		let yellow_cmyk = Cmyk::from(yellow);
 		println!("Testing Rgb to Hsl. yellow = {:?}", yellow_hsl);
 		assert!(nearly_equal(yellow_hsl.hue(), 60f32));
 		assert!(nearly_equal(yellow_hsl.saturation() * 100f32, 100f32));
 		assert!(nearly_equal(yellow_hsl.lightness() * 100f32, 50f32));
 		println!("Testing Rgb to Cmyk. yellow = {:?}", yellow_cmyk);
 		assert_eq!(yellow_cmyk.c, 0);
 		assert_eq!(yellow_cmyk.m, 0);
 		assert_eq!(yellow_cmyk.y, 255);
 		assert_eq!(yellow_cmyk.k, 0);
 		let yellow_rgb_a = Rgb::from(yellow_hsl);
 		println!("Testing Hsl to Rgb. yellow = {:?}", yellow_rgb_a);
 		assert_eq!(yellow_rgb_a, yellow);
 		let yellow_rgb_b = Rgb::from(yellow_cmyk);
 		println!("Testing Cmyk to Rgb. yellow = {:?}", yellow_rgb_b);
 		assert_eq!(yellow_rgb_b, yellow);

 		// Cyan conversions.
 		let cyan = Rgb::from(0x00FFFF);
 		let cyan_hsl = Hsl::from(cyan);
 		let cyan_cmyk = Cmyk::from(cyan);
 		println!("Testing Rgb to Hsl. cyan = {:?}", cyan_hsl);
 		assert!(nearly_equal(cyan_hsl.hue(), 180f32));
 		assert!(nearly_equal(cyan_hsl.saturation() * 100f32, 100f32));
 		assert!(nearly_equal(cyan_hsl.lightness() * 100f32, 50f32));
 		println!("Testing Rgb to Cmyk. cyan = {:?}", cyan_cmyk);
 		assert_eq!(cyan_cmyk.c, 255);
 		assert_eq!(cyan_cmyk.m, 0);
 		assert_eq!(cyan_cmyk.y, 0);
 		assert_eq!(cyan_cmyk.k, 0);
 		let cyan_rgb_a = Rgb::from(cyan_hsl);
 		println!("Testing Hsl to Rgb. cyan = {:?}", cyan_rgb_a);
 		assert_eq!(cyan_rgb_a, cyan);
 		let cyan_rgb_b = Rgb::from(cyan_cmyk);
 		println!("Testing Cmyk to Rgb. cyan = {:?}", cyan_rgb_b);
 		assert_eq!(cyan_rgb_b, cyan);

 		// Magenta conversions.
 		let magenta = Rgb::from(0xFF00FF);
 		let magenta_hsl = Hsl::from(magenta);
 		let magenta_cmyk = Cmyk::from(magenta);
 		println!("Testing Rgb to Hsl. magenta = {:?}", magenta_hsl);
 		assert!(nearly_equal(magenta_hsl.hue(), 300f32));
 		assert!(nearly_equal(magenta_hsl.saturation() * 100f32, 100f32));
 		assert!(nearly_equal(magenta_hsl.lightness() * 100f32, 50f32));
 		println!("Testing Rgb to Cmyk. magenta = {:?}", magenta_cmyk);
 		assert_eq!(magenta_cmyk.c, 0);
 		assert_eq!(magenta_cmyk.m, 255);
 		assert_eq!(magenta_cmyk.y, 0);
 		assert_eq!(magenta_cmyk.k, 0);
 		let magenta_rgb_a = Rgb::from(magenta_hsl);
 		println!("Testing Hsl to Rgb. magenta = {:?}", magenta_rgb_a);
 		assert_eq!(magenta_rgb_a, magenta);
 		let magenta_rgb_b = Rgb::from(magenta_cmyk);
 		println!("Testing Cmyk to Rgb. magenta = {:?}", magenta_rgb_b);
 		assert_eq!(magenta_rgb_b, magenta);

 		// Silver conversions.
 		let silver = Rgb::from(0xC0C0C0);
 		let silver_hsl = Hsl::from(silver);
 		let silver_cmyk = Cmyk::from(silver);
 		println!("Testing Rgb to Hsl. silver = {:?}", silver_hsl);
 		assert!(nearly_equal(silver_hsl.hue(), 0f32));
 		assert!(nearly_equal(silver_hsl.saturation() * 100f32, 0f32));
 		assert!(nearly_equal(silver_hsl.lightness() * 100f32, 75f32));
 		println!("Testing Rgb to Cmyk. silver = {:?}", silver_cmyk);
 		assert_eq!(silver_cmyk.c, 0);
 		assert_eq!(silver_cmyk.m, 0);
 		assert_eq!(silver_cmyk.y, 0);
 		assert_eq!(silver_cmyk.k, 63);
 		let silver_rgb_a = Rgb::from(silver_hsl);
 		println!("Testing Hsl to Rgb. silver = {:?}", silver_rgb_a);
 		assert_eq!(silver_rgb_a, silver);
 		let silver_rgb_b = Rgb::from(silver_cmyk);
 		println!("Testing Cmyk to Rgb. silver = {:?}", silver_rgb_b);
 		assert_eq!(silver_rgb_b, silver);

 		// Gray conversions.
 		let gray = Rgb::from(0x808080);
 		let gray_hsl = Hsl::from(gray);
 		let gray_cmyk = Cmyk::from(gray);
 		println!("Testing Rgb to Hsl. gray = {:?}", gray_hsl);
 		assert!(nearly_equal(gray_hsl.hue(), 0f32));
 		assert!(nearly_equal(gray_hsl.saturation() * 100f32, 0f32));
 		assert!(nearly_equal(gray_hsl.lightness() * 100f32, 50f32));
 		println!("Testing Rgb to Cmyk. gray = {:?}", gray_cmyk);
 		assert_eq!(gray_cmyk.c, 0);
 		assert_eq!(gray_cmyk.m, 0);
 		assert_eq!(gray_cmyk.y, 0);
 		assert_eq!(gray_cmyk.k, 126);
 		let gray_rgb_a = Rgb::from(gray_hsl);
 		println!("Testing Hsl to Rgb. gray = {:?}", gray_rgb_a);
 		assert_eq!(gray_rgb_a, gray);
 		let gray_rgb_b = Rgb::from(gray_cmyk);
 		println!("Testing Cmyk to Rgb. gray = {:?}", gray_rgb_b);
 		assert_eq!(gray_rgb_b, gray);

 		// Maroon conversions.
 		let maroon = Rgb::from(0x800000);
 		let maroon_hsl = Hsl::from(maroon);
 		let maroon_cmyk = Cmyk::from(maroon);
 		println!("Testing Rgb to Hsl. maroon = {:?}", maroon_hsl);
 		assert!(nearly_equal(maroon_hsl.hue(), 0f32));
 		assert!(nearly_equal(maroon_hsl.saturation() * 100f32, 100f32));
 		assert!(nearly_equal(maroon_hsl.lightness() * 100f32, 25f32));
 		println!("Testing Rgb to Cmyk. maroon = {:?}", maroon_cmyk);
 		assert_eq!(maroon_cmyk.c, 0);
 		assert_eq!(maroon_cmyk.m, 255);
 		assert_eq!(maroon_cmyk.y, 255);
 		assert_eq!(maroon_cmyk.k, 126);
 		let maroon_rgb_a = Rgb::from(maroon_hsl);
 		println!("Testing Hsl to Rgb. maroon = {:?}", maroon_rgb_a);
 		assert_eq!(maroon_rgb_a, maroon);
 		let maroon_rgb_b = Rgb::from(maroon_cmyk);
 		println!("Testing Cmyk to Rgb. maroon = {:?}", maroon_rgb_b);
 		assert_eq!(maroon_rgb_b, maroon);

 		// Olive conversions.
 		let olive = Rgb::from(0x808000);
 		let olive_hsl = Hsl::from(olive);
 		let olive_cmyk = Cmyk::from(olive);
 		println!("Testing Rgb to Hsl. olive = {:?}", olive_hsl);
 		assert!(nearly_equal(olive_hsl.hue(), 60f32));
 		assert!(nearly_equal(olive_hsl.saturation() * 100f32, 100f32));
 		assert!(nearly_equal(olive_hsl.lightness() * 100f32, 25f32));
 		println!("Testing Rgb to Cmyk. olive = {:?}", olive_cmyk);
 		assert_eq!(olive_cmyk.c, 0);
 		assert_eq!(olive_cmyk.m, 0);
 		assert_eq!(olive_cmyk.y, 255);
 		assert_eq!(olive_cmyk.k, 126);
 		let olive_rgb_a = Rgb::from(olive_hsl);
 		println!("Testing Hsl to Rgb. olive = {:?}", olive_rgb_a);
 		assert_eq!(olive_rgb_a, olive);
 		let olive_rgb_b = Rgb::from(olive_cmyk);
 		println!("Testing Cmyk to Rgb. olive = {:?}", olive_rgb_b);
 		assert_eq!(olive_rgb_b, olive);

 		// Green conversions.
 		let green = Rgb::from(0x008000);
 		let green_hsl = Hsl::from(green);
 		let green_cmyk = Cmyk::from(green);
 		println!("Testing Rgb to Hsl. green = {:?}", green_hsl);
 		assert!(nearly_equal(green_hsl.hue(), 120f32));
 		assert!(nearly_equal(green_hsl.saturation() * 100f32, 100f32));
 		assert!(nearly_equal(green_hsl.lightness() * 100f32, 25f32));
 		println!("Testing Rgb to Cmyk. green = {:?}", green_cmyk);
 		assert_eq!(green_cmyk.c, 255);
 		assert_eq!(green_cmyk.m, 0);
 		assert_eq!(green_cmyk.y, 255);
 		assert_eq!(green_cmyk.k, 126);
 		let green_rgb_a = Rgb::from(green_hsl);
 		println!("Testing Hsl to Rgb. green = {:?}", green_rgb_a);
 		assert_eq!(green_rgb_a, green);
 		let green_rgb_b = Rgb::from(green_cmyk);
 		println!("Testing Cmyk to Rgb. green = {:?}", green_rgb_b);
 		assert_eq!(green_rgb_b, green);

 		// Purple conversions.
 		let purple = Rgb::from(0x800080);
 		let purple_hsl = Hsl::from(purple);
 		let purple_cmyk = Cmyk::from(purple);
 		println!("Testing Rgb to Hsl. purple = {:?}", purple_hsl);
 		assert!(nearly_equal(purple_hsl.hue(), 300f32));
 		assert!(nearly_equal(purple_hsl.saturation() * 100f32, 100f32));
 		assert!(nearly_equal(purple_hsl.lightness() * 100f32, 25f32));
 		println!("Testing Rgb to Cmyk. purple = {:?}", purple_cmyk);
 		assert_eq!(purple_cmyk.c, 0);
 		assert_eq!(purple_cmyk.m, 255);
 		assert_eq!(purple_cmyk.y, 0);
 		assert_eq!(purple_cmyk.k, 126);
 		let purple_rgb_a = Rgb::from(purple_hsl);
 		println!("Testing Hsl to Rgb. purple = {:?}", purple_rgb_a);
 		assert_eq!(purple_rgb_a, purple);
 		let purple_rgb_b = Rgb::from(purple_cmyk);
 		println!("Testing Cmyk to Rgb. purple = {:?}", purple_rgb_b);
 		assert_eq!(purple_rgb_b, purple);

 		// Teal conversions.
 		let teal = Rgb::from(0x008080);
 		let teal_hsl = Hsl::from(teal);
 		let teal_cmyk = Cmyk::from(teal);
 		println!("Testing Rgb to Hsl. teal = {:?}", teal_hsl);
 		assert!(nearly_equal(teal_hsl.hue(), 180f32));
 		assert!(nearly_equal(teal_hsl.saturation() * 100f32, 100f32));
 		assert!(nearly_equal(teal_hsl.lightness() * 100f32, 25f32));
 		println!("Testing Rgb to Cmyk. teal = {:?}", teal_cmyk);
 		assert_eq!(teal_cmyk.c, 255);
 		assert_eq!(teal_cmyk.m, 0);
 		assert_eq!(teal_cmyk.y, 0);
 		assert_eq!(teal_cmyk.k, 126);
 		let teal_rgb_a = Rgb::from(teal_hsl);
 		println!("Testing Hsl to Rgb. teal = {:?}", teal_rgb_a);
 		assert_eq!(teal_rgb_a, teal);
 		let teal_rgb_b = Rgb::from(teal_cmyk);
 		println!("Testing Cmyk to Rgb. teal = {:?}", teal_rgb_b);
 		assert_eq!(teal_rgb_b, teal);

 		// Navy conversions.
 		let navy = Rgb::from(0x000080);
 		let navy_hsl = Hsl::from(navy);
 		let navy_cmyk = Cmyk::from(navy);
 		println!("Testing Rgb to Hsl. navy = {:?}", navy_hsl);
 		assert!(nearly_equal(navy_hsl.hue(), 240f32));
 		assert!(nearly_equal(navy_hsl.saturation() * 100f32, 100f32));
 		assert!(nearly_equal(navy_hsl.lightness() * 100f32, 25f32));
 		println!("Testing Rgb to Cmyk. navy = {:?}", navy_cmyk);
 		assert_eq!(navy_cmyk.c, 255);
 		assert_eq!(navy_cmyk.m, 255);
 		assert_eq!(navy_cmyk.y, 0);
 		assert_eq!(navy_cmyk.k, 126);
 		let navy_rgb_a = Rgb::from(navy_hsl);
 		println!("Testing Hsl to Rgb. navy = {:?}", navy_rgb_a);
 		assert_eq!(navy_rgb_a, navy);
 		let navy_rgb_b = Rgb::from(navy_cmyk);
 		println!("Testing Cmyk to Rgb. navy = {:?}", navy_rgb_b);
 		assert_eq!(navy_rgb_b, navy);
 	}
}
