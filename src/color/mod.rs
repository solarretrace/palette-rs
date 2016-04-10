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
pub mod hsv;

#[warn(missing_docs)]
pub mod rgb;

#[warn(missing_docs)]
pub mod xyz;

pub use color::cmyk::*;
pub use color::hsl::*;
pub use color::hsv::*;
pub use color::rgb::*;
pub use color::xyz::*;

use utilities::clamped;
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
	pub fn hsl_saturation(&self) -> f32 {
		Hsl::from(self.rgb).saturation()
	}

	/// Returns the saturation.
	pub fn hsv_saturation(&self) -> f32 {
		Hsv::from(self.rgb).saturation()
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
		let mut t = Hsv::from(self.rgb);
		t.set_hue(value);
		self.rgb = Rgb::from(t);
	}

	/// Shifts the hue by the given number of degrees.
	pub fn shift_hue(&mut self, degrees: f32) {
		let h = self.hue();
		self.set_hue(h + degrees);
	}

	/// Sets the saturation.
	pub fn set_hsl_saturation(&mut self, value: f32) {
		let mut t = Hsl::from(self.rgb);
		t.set_saturation(value);
		self.rgb = Rgb::from(t);
	}

	/// Sets the saturation.
	pub fn set_hsv_saturation(&mut self, value: f32) {
		let mut t = Hsv::from(self.rgb);
		t.set_saturation(value);
		self.rgb = Rgb::from(t);
	}

	/// Saturates the color in the HSL color space by the given proportion.
	pub fn hsl_saturate(&mut self, value: f32) {
		let s = self.hsl_saturation();
		let v = clamped(value, 0.0, 1.0);
		self.set_hsl_saturation(s + (s * v));
	}

	/// Desaturates the color in the HSL color space by the given proportion.
	pub fn hsl_desaturate(&mut self, value: f32) {
		let s = self.hsl_saturation();
		let v = clamped(value, 0.0, 1.0);
		self.set_hsl_saturation(s - (s * v));
	}

	/// Saturates the color in the HSV color space by the given proportion.
	pub fn hsv_saturate(&mut self, value: f32) {
		let s = self.hsv_saturation();
		let v = clamped(value, 0.0, 1.0);
		self.set_hsv_saturation(s + (s * v));
	}

	/// Desaturates the color in the HSV color space by the given proportion.
	pub fn hsv_desaturate(&mut self, value: f32) {
		let s = self.hsv_saturation();
		let v = clamped(value, 0.0, 1.0);
		self.set_hsv_saturation(s - (s * v));
	}

	/// Sets the lightness.
	pub fn set_lightness(&mut self, value: f32) {
		let mut t = Hsl::from(self.rgb);
		t.set_lightness(value);
		self.rgb = Rgb::from(t);
	}


	/// Lightens the color by the given proportion.
	pub fn lighten(&mut self, value: f32) {
		let l = self.lightness();
		let v = clamped(value, 0.0, 1.0);
		self.set_lightness(l + (l * v));
	}

	/// Darkens the color by the given proportion.
	pub fn darken(&mut self, value: f32) {
		let l = self.lightness();
		let v = clamped(value, 0.0, 1.0);
		self.set_lightness(l - (l * v));
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

	/// Returns an array containing the [H, S, V] components.
	pub fn hsv_components(&self) -> [f32; 3] {
		Hsv::from(self.rgb).components()
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

impl From<Hsv> for Color {
	fn from(hsv: Hsv) -> Color {
		Color {rgb: Rgb::from(hsv)}
	}
}

impl From<Xyz> for Color {
	fn from(xyz: Xyz) -> Color {
		Color {rgb: Rgb::from(xyz)}
	}
}



////////////////////////////////////////////////////////////////////////////////
// Test Module
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::{Cmyk, Hsl, Hsv, Rgb};
    use super::super::utilities::close;

    const UNIT: f32 = 1.0 / 255.0;

	/// Tests color conversions for the color black.
	#[test]
	fn color_conversions_black() {
		let black = Rgb::from(0x000000);
		let black_hsl = Hsl::from(black);
		let black_hsv = Hsv::from(black);
		let black_cmyk = Cmyk::from(black);
		
		println!("Testing Rgb to Hsl. black = {:?}", black_hsl);
		assert!(close(black_hsl.hue(), 0.0, UNIT));
		assert!(close(black_hsl.saturation(), 0.0, UNIT));
		assert!(close(black_hsl.lightness(), 0.0, UNIT));
		
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
		
		println!("Testing Rgb to Hsv. black = {:?}", black_hsv);
		assert!(close(black_hsv.hue(), 0.0, UNIT));
		assert!(close(black_hsv.saturation(), 0.0, UNIT));
		assert!(close(black_hsv.value(), 0.0, UNIT));
 	}

	/// Tests color conversions for the color white.
	#[test]
	fn color_conversions_white() {
		let white = Rgb::from(0xFFFFFF);
		let white_hsl = Hsl::from(white);
		let white_hsv = Hsv::from(white);
		let white_cmyk = Cmyk::from(white);
		
		println!("Testing Rgb to Hsl. white = {:?}", white_hsl);
		assert!(close(white_hsl.hue(), 0.0, UNIT));
		assert!(close(white_hsl.saturation(), 0.0, UNIT));
		assert!(close(white_hsl.lightness(), 1.0, UNIT));
		
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
		
		println!("Testing Rgb to Hsv. white = {:?}", white_hsv);
		assert!(close(white_hsv.hue(), 0.0, UNIT));
		assert!(close(white_hsv.saturation(), 0.0, UNIT));
		assert!(close(white_hsv.value(), 1.0, UNIT));
 	}

	/// Tests color conversions for the color red.
	#[test]
	fn color_conversions_red() {
		let red = Rgb::from(0xFF0000);
		let red_hsl = Hsl::from(red);
		let red_hsv = Hsv::from(red);
		let red_cmyk = Cmyk::from(red);
		
		println!("Testing Rgb to Hsl. red = {:?}", red_hsl);
		assert!(close(red_hsl.hue(), 0.0, UNIT));
		assert!(close(red_hsl.saturation() , 1.0, UNIT));
		assert!(close(red_hsl.lightness(), 0.5, UNIT));
		
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
		
		println!("Testing Rgb to Hsv. red = {:?}", red_hsv);
		assert!(close(red_hsv.hue(), 0.0, UNIT));
		assert!(close(red_hsv.saturation(), 1.0, UNIT));
		assert!(close(red_hsv.value(), 1.0, UNIT));
 	}

	/// Tests color conversions for the color lime.
	#[test]
	fn color_conversions_lime() {
		let lime = Rgb::from(0x00FF00);
		let lime_hsl = Hsl::from(lime);
		let lime_hsv = Hsv::from(lime);
		let lime_cmyk = Cmyk::from(lime);
		
		println!("Testing Rgb to Hsl. lime = {:?}", lime_hsl);
		assert!(close(lime_hsl.hue(), 120.0, UNIT));
		assert!(close(lime_hsl.saturation(), 1.0, UNIT));
		assert!(close(lime_hsl.lightness(), 0.5, UNIT));
		
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
		
		println!("Testing Rgb to Hsv. lime = {:?}", lime_hsv);
		assert!(close(lime_hsv.hue(), 120.0, UNIT));
		assert!(close(lime_hsv.saturation(), 1.0, UNIT));
		assert!(close(lime_hsv.value(), 1.0, UNIT));
 	}

	/// Tests color conversions for the color blue.
	#[test]
	fn color_conversions_blue() {
		let blue = Rgb::from(0x0000FF);
		let blue_hsl = Hsl::from(blue);
		let blue_hsv = Hsv::from(blue);
		let blue_cmyk = Cmyk::from(blue);
		
		println!("Testing Rgb to Hsl. blue = {:?}", blue_hsl);
		assert!(close(blue_hsl.hue(), 240.0, UNIT));
		assert!(close(blue_hsl.saturation(), 1.0, UNIT));
		assert!(close(blue_hsl.lightness(), 0.5, UNIT));
		
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
		
		println!("Testing Rgb to Hsv. blue = {:?}", blue_hsv);
		assert!(close(blue_hsv.hue(), 240.0, UNIT));
		assert!(close(blue_hsv.saturation(), 1.0, UNIT));
		assert!(close(blue_hsv.value(), 1.0, UNIT));
 	}

	/// Tests color conversions for the color yellow.
	#[test]
	fn color_conversions_yellow() {
		let yellow = Rgb::from(0xFFFF00);
		let yellow_hsl = Hsl::from(yellow);
		let yellow_hsv = Hsv::from(yellow);
		let yellow_cmyk = Cmyk::from(yellow);
		
		println!("Testing Rgb to Hsl. yellow = {:?}", yellow_hsl);
		assert!(close(yellow_hsl.hue(), 60.0, UNIT));
		assert!(close(yellow_hsl.saturation(), 1.0, UNIT));
		assert!(close(yellow_hsl.lightness(), 0.5, UNIT));
		
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
		
		println!("Testing Rgb to Hsv. yellow = {:?}", yellow_hsv);
		assert!(close(yellow_hsv.hue(), 60.0, UNIT));
		assert!(close(yellow_hsv.saturation(), 1.0, UNIT));
		assert!(close(yellow_hsv.value(), 1.0, UNIT));
 	}

	/// Tests color conversions for the color cyan.
	#[test]
	fn color_conversions_cyan() {
		let cyan = Rgb::from(0x00FFFF);
		let cyan_hsl = Hsl::from(cyan);
		let cyan_hsv = Hsv::from(cyan);
		let cyan_cmyk = Cmyk::from(cyan);
		
		println!("Testing Rgb to Hsl. cyan = {:?}", cyan_hsl);
		assert!(close(cyan_hsl.hue(), 180.0, UNIT));
		assert!(close(cyan_hsl.saturation(), 1.0, UNIT));
		assert!(close(cyan_hsl.lightness(), 0.5, UNIT));
		
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
		
		println!("Testing Rgb to Hsv. cyan = {:?}", cyan_hsv);
		assert!(close(cyan_hsv.hue(), 180.0, UNIT));
		assert!(close(cyan_hsv.saturation(), 1.0, UNIT));
		assert!(close(cyan_hsv.value(), 1.0, UNIT));
 	}

	/// Tests color conversions for the color magenta.
	#[test]
	fn color_conversions_magenta() {
		let magenta = Rgb::from(0xFF00FF);
		let magenta_hsl = Hsl::from(magenta);
		let magenta_hsv = Hsv::from(magenta);
		let magenta_cmyk = Cmyk::from(magenta);
		
		println!("Testing Rgb to Hsl. magenta = {:?}", magenta_hsl);
		assert!(close(magenta_hsl.hue(), 300.0, UNIT));
		assert!(close(magenta_hsl.saturation(), 1.0, UNIT));
		assert!(close(magenta_hsl.lightness(), 0.5, UNIT));
		
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
		
		println!("Testing Rgb to Hsv. magenta = {:?}", magenta_hsv);
		assert!(close(magenta_hsv.hue(), 300.0, UNIT));
		assert!(close(magenta_hsv.saturation(), 1.0, UNIT));
		assert!(close(magenta_hsv.value(), 1.0, UNIT));
 	}

	/// Tests color conversions for the color silver.
	#[test]
	fn color_conversions_silver() {
		let silver = Rgb::from(0xC0C0C0);
		let silver_hsl = Hsl::from(silver);
		let silver_hsv = Hsv::from(silver);
		let silver_cmyk = Cmyk::from(silver);
		
		println!("Testing Rgb to Hsl. silver = {:?}", silver_hsl);
		assert!(close(silver_hsl.hue(), 0.0, UNIT));
		assert!(close(silver_hsl.saturation(), 0.0, UNIT));
		assert!(close(silver_hsl.lightness(), 0.75, UNIT));
		
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
		
		println!("Testing Rgb to Hsv. silver = {:?}", silver_hsv);
		assert!(close(silver_hsv.hue(), 0.0, UNIT));
		assert!(close(silver_hsv.saturation(), 0.0, UNIT));
		assert!(close(silver_hsv.value(), 0.75, UNIT));
 	}

	/// Tests color conversions for the color gray.
	#[test]
	fn color_conversions_gray() {
		let gray = Rgb::from(0x808080);
		let gray_hsl = Hsl::from(gray);
		let gray_hsv = Hsv::from(gray);
		let gray_cmyk = Cmyk::from(gray);
		
		println!("Testing Rgb to Hsl. gray = {:?}", gray_hsl);
		assert!(close(gray_hsl.hue(), 0.0, UNIT));
		assert!(close(gray_hsl.saturation(), 0.0, UNIT));
		assert!(close(gray_hsl.lightness(), 0.5, UNIT));
		
		println!("Testing Rgb to Cmyk. gray = {:?}", gray_cmyk);
		assert_eq!(gray_cmyk.c, 0);
		assert_eq!(gray_cmyk.m, 0);
		assert_eq!(gray_cmyk.y, 0);
		assert_eq!(gray_cmyk.k, 127);
		
		let gray_rgb_a = Rgb::from(gray_hsl);
		println!("Testing Hsl to Rgb. gray = {:?}", gray_rgb_a);
		assert_eq!(gray_rgb_a, gray);
		
		let gray_rgb_b = Rgb::from(gray_cmyk);
		println!("Testing Cmyk to Rgb. gray = {:?}", gray_rgb_b);
		assert_eq!(gray_rgb_b, gray);
		
		println!("Testing Rgb to Hsv. gray = {:?}", gray_hsv);
		assert!(close(gray_hsv.hue(), 0.0, UNIT));
		assert!(close(gray_hsv.saturation(), 0.0, UNIT));
		assert!(close(gray_hsv.value(), 0.50, UNIT));
 	}

	/// Tests color conversions for the color maroon.
	#[test]
	fn color_conversions_maroon() {
		let maroon = Rgb::from(0x800000);
		let maroon_hsl = Hsl::from(maroon);
		let maroon_hsv = Hsv::from(maroon);
		let maroon_cmyk = Cmyk::from(maroon);
		
		println!("Testing Rgb to Hsl. maroon = {:?}", maroon_hsl);
		assert!(close(maroon_hsl.hue(), 0.0, UNIT));
		assert!(close(maroon_hsl.saturation(), 1.0, UNIT));
		assert!(close(maroon_hsl.lightness(), 0.25, UNIT));
		
		println!("Testing Rgb to Cmyk. maroon = {:?}", maroon_cmyk);
		assert_eq!(maroon_cmyk.c, 0);
		assert_eq!(maroon_cmyk.m, 255);
		assert_eq!(maroon_cmyk.y, 255);
		assert_eq!(maroon_cmyk.k, 127);
		
		let maroon_rgb_a = Rgb::from(maroon_hsl);
		println!("Testing Hsl to Rgb. maroon = {:?}", maroon_rgb_a);
		assert_eq!(maroon_rgb_a, maroon);
		
		let maroon_rgb_b = Rgb::from(maroon_cmyk);
		println!("Testing Cmyk to Rgb. maroon = {:?}", maroon_rgb_b);
		assert_eq!(maroon_rgb_b, maroon);
		
		println!("Testing Rgb to Hsv. maroon = {:?}", maroon_hsv);
		assert!(close(maroon_hsv.hue(), 0.0, UNIT));
		assert!(close(maroon_hsv.saturation(), 1.0, UNIT));
		assert!(close(maroon_hsv.value(), 0.5, UNIT));
 	}

	/// Tests color conversions for the color olive.
	#[test]
	fn color_conversions_olive() {
		let olive = Rgb::from(0x808000);
		let olive_hsl = Hsl::from(olive);
		let olive_hsv = Hsv::from(olive);
		let olive_cmyk = Cmyk::from(olive);
		
		println!("Testing Rgb to Hsl. olive = {:?}", olive_hsl);
		assert!(close(olive_hsl.hue(), 60.0, UNIT));
		assert!(close(olive_hsl.saturation(), 1.0, UNIT));
		assert!(close(olive_hsl.lightness(), 0.25, UNIT));
		
		println!("Testing Rgb to Cmyk. olive = {:?}", olive_cmyk);
		assert_eq!(olive_cmyk.c, 0);
		assert_eq!(olive_cmyk.m, 0);
		assert_eq!(olive_cmyk.y, 255);
		assert_eq!(olive_cmyk.k, 127);
		
		let olive_rgb_a = Rgb::from(olive_hsl);
		println!("Testing Hsl to Rgb. olive = {:?}", olive_rgb_a);
		assert_eq!(olive_rgb_a, olive);
		
		let olive_rgb_b = Rgb::from(olive_cmyk);
		println!("Testing Cmyk to Rgb. olive = {:?}", olive_rgb_b);
		assert_eq!(olive_rgb_b, olive);
		
		println!("Testing Rgb to Hsv. olive = {:?}", olive_hsv);
		assert!(close(olive_hsv.hue(), 60.0, UNIT));
		assert!(close(olive_hsv.saturation(), 1.0, UNIT));
		assert!(close(olive_hsv.value(), 0.5, UNIT));
 	}

	/// Tests color conversions for the color green.
	#[test]
	fn color_conversions_green() {
		let green = Rgb::from(0x008000);
		let green_hsl = Hsl::from(green);
		let green_hsv = Hsv::from(green);
		let green_cmyk = Cmyk::from(green);
		
		println!("Testing Rgb to Hsl. green = {:?}", green_hsl);
		assert!(close(green_hsl.hue(), 120.0, UNIT));
		assert!(close(green_hsl.saturation(), 1.0, UNIT));
		assert!(close(green_hsl.lightness(), 0.25, UNIT));
		
		println!("Testing Rgb to Cmyk. green = {:?}", green_cmyk);
		assert_eq!(green_cmyk.c, 255);
		assert_eq!(green_cmyk.m, 0);
		assert_eq!(green_cmyk.y, 255);
		assert_eq!(green_cmyk.k, 127);
		
		let green_rgb_a = Rgb::from(green_hsl);
		println!("Testing Hsl to Rgb. green = {:?}", green_rgb_a);
		assert_eq!(green_rgb_a, green);
		
		let green_rgb_b = Rgb::from(green_cmyk);
		println!("Testing Cmyk to Rgb. green = {:?}", green_rgb_b);
		assert_eq!(green_rgb_b, green);
		
		println!("Testing Rgb to Hsv. green = {:?}", green_hsv);
		assert!(close(green_hsv.hue(), 120.0, UNIT));
		assert!(close(green_hsv.saturation(), 1.0, UNIT));
		assert!(close(green_hsv.value(), 0.5, UNIT));
 	}

	/// Tests color conversions for the color purple.
	#[test]
	fn color_conversions_purple() {
		let purple = Rgb::from(0x800080);
		let purple_hsl = Hsl::from(purple);
		let purple_hsv = Hsv::from(purple);
		let purple_cmyk = Cmyk::from(purple);
		
		println!("Testing Rgb to Hsl. purple = {:?}", purple_hsl);
		assert!(close(purple_hsl.hue(), 300.0, UNIT));
		assert!(close(purple_hsl.saturation(), 1.0, UNIT));
		assert!(close(purple_hsl.lightness(), 0.25, UNIT));
		
		println!("Testing Rgb to Cmyk. purple = {:?}", purple_cmyk);
		assert_eq!(purple_cmyk.c, 0);
		assert_eq!(purple_cmyk.m, 255);
		assert_eq!(purple_cmyk.y, 0);
		assert_eq!(purple_cmyk.k, 127);
		
		let purple_rgb_a = Rgb::from(purple_hsl);
		println!("Testing Hsl to Rgb. purple = {:?}", purple_rgb_a);
		assert_eq!(purple_rgb_a, purple);
		
		let purple_rgb_b = Rgb::from(purple_cmyk);
		println!("Testing Cmyk to Rgb. purple = {:?}", purple_rgb_b);
		assert_eq!(purple_rgb_b, purple);
		
		println!("Testing Rgb to Hsv. purple = {:?}", purple_hsv);
		assert!(close(purple_hsv.hue(), 300.0, UNIT));
		assert!(close(purple_hsv.saturation(), 1.0, UNIT));
		assert!(close(purple_hsv.value(), 0.5, UNIT));
 	}

	/// Tests color conversions for the color teal.
	#[test]
	fn color_conversions_teal() {
		let teal = Rgb::from(0x008080);
		let teal_hsl = Hsl::from(teal);
		let teal_hsv = Hsv::from(teal);
		let teal_cmyk = Cmyk::from(teal);
		
		println!("Testing Rgb to Hsl. teal = {:?}", teal_hsl);
		assert!(close(teal_hsl.hue(), 180.0, UNIT));
		assert!(close(teal_hsl.saturation(), 1.0, UNIT));
		assert!(close(teal_hsl.lightness(), 0.25, UNIT));
		
		println!("Testing Rgb to Cmyk. teal = {:?}", teal_cmyk);
		assert_eq!(teal_cmyk.c, 255);
		assert_eq!(teal_cmyk.m, 0);
		assert_eq!(teal_cmyk.y, 0);
		assert_eq!(teal_cmyk.k, 127);
		
		let teal_rgb_a = Rgb::from(teal_hsl);
		println!("Testing Hsl to Rgb. teal = {:?}", teal_rgb_a);
		assert_eq!(teal_rgb_a, teal);
		
		let teal_rgb_b = Rgb::from(teal_cmyk);
		println!("Testing Cmyk to Rgb. teal = {:?}", teal_rgb_b);
		assert_eq!(teal_rgb_b, teal);
		
		println!("Testing Rgb to Hsv. teal = {:?}", teal_hsv);
		assert!(close(teal_hsv.hue(), 180.0, UNIT));
		assert!(close(teal_hsv.saturation(), 1.0, UNIT));
		assert!(close(teal_hsv.value(), 0.5, UNIT));
 	}

	/// Tests color conversions for the color navy.
	#[test]
	fn color_conversions_navy() {
		let navy = Rgb::from(0x000080);
		let navy_hsl = Hsl::from(navy);
		let navy_hsv = Hsv::from(navy);
		let navy_cmyk = Cmyk::from(navy);
		
		println!("Testing Rgb to Hsl. navy = {:?}", navy_hsl);
		assert!(close(navy_hsl.hue(), 240.0, UNIT));
		assert!(close(navy_hsl.saturation(), 1.0, UNIT));
		assert!(close(navy_hsl.lightness(), 0.25, UNIT));
		
		println!("Testing Rgb to Cmyk. navy = {:?}", navy_cmyk);
		assert_eq!(navy_cmyk.c, 255);
		assert_eq!(navy_cmyk.m, 255);
		assert_eq!(navy_cmyk.y, 0);
		assert_eq!(navy_cmyk.k, 127);
		
		let navy_rgb_a = Rgb::from(navy_hsl);
		println!("Testing Hsl to Rgb. navy = {:?}", navy_rgb_a);
		assert_eq!(navy_rgb_a, navy);
		
		let navy_rgb_b = Rgb::from(navy_cmyk);
		println!("Testing Cmyk to Rgb. navy = {:?}", navy_rgb_b);
		assert_eq!(navy_rgb_b, navy);
		
		println!("Testing Rgb to Hsv. navy = {:?}", navy_hsv);
		assert!(close(navy_hsv.hue(), 240.0, UNIT));
		assert!(close(navy_hsv.saturation(), 1.0, UNIT));
		assert!(close(navy_hsv.value(), 0.5, UNIT));
 	}
}
