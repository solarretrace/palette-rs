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
extern crate color;
#[macro_use] 
extern crate conrod;
extern crate find_folder;
extern crate piston_window;
extern crate image;


use color::Rgb;
use conrod::{Theme, Widget};
use piston_window::{
	EventLoop, 
	Glyphs, 
	PistonWindow, 
	UpdateEvent, 
	WindowSettings,
};
use rampeditor::*;
use rampeditor::gui::*;


fn main() {
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


    // Construct the window.
    let mut window: PistonWindow =
    	WindowSettings::new("Rampeditor 0.1.0", [600, 600])
            .exit_on_esc(true)
            .vsync(true)
            .build()
            .expect("new window");

    // construct our `Ui`.
    let mut ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets")
            .expect("assets directory");
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(
        	&font_path, 
        	window.factory.clone()
        );
        Ui::new(glyph_cache.expect("glyph cache"), theme)
    };


    let mut editor = Editor::new(pal);

    window.set_ups(60);

    // Poll events from the window.
    while let Some(event) = window.next() {
        ui.handle_event(event.clone());
        event.update(|_| ui.set_widgets(|mut ui| 
        	gui::set_widgets(&mut ui, &mut editor, &mut window))
        );
        window.draw_2d(&event, |c, g| {
            ui.draw_if_changed(c, g);
        });
    }
}
