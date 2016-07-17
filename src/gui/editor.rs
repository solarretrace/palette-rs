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
//! The main palette editor.
//!
////////////////////////////////////////////////////////////////////////////////

use palette::Palette;
// use Color;
use super::picker::ColorPicker;

use conrod::{
	color,
    Canvas,
    Widget,
    Frameable,
    Positionable,
    Colorable,
    Sizeable,
};
use conrod;
use piston_window::PistonWindow;
use piston_window;


/// The editor's back-end.
pub type Backend = (
	<piston_window::G2d<'static> as conrod::Graphics>::Texture, 
	piston_window::Glyphs
);
/// The Conrod `Ui` for the back-end.
pub type Ui = conrod::Ui<Backend>;
/// The Conrod `UiCell` for the back-end.
pub type UiCell<'a> = conrod::UiCell<'a, Backend>;


////////////////////////////////////////////////////////////////////////////////
// Editor
////////////////////////////////////////////////////////////////////////////////
/// The editor's state.
pub struct Editor {
	/// The width of the editor's frames.
	pub frame_width: f64,
	/// The current palette.
	pub palette: Palette,

}

impl Editor {
	/// Constructs a new `Editor` for the given palette.
	pub fn new(pal: Palette) -> Self {
		Editor {
			frame_width: 1.0,
			palette: pal,
		}
	}
}

/// Set all `Widget`s in the user interface.
pub fn set_widgets(
    ui: &mut UiCell, 
    editor: &mut Editor, 
    window: &mut PistonWindow) 
{
	// Root canvas.
    Canvas::new()
        .frame(1.0)
        .pad(30.0)
        .color(color::rgb(20.0, 0.0, 0.0))
        .scroll_kids()
        .set(CANVAS, ui);

    ColorPicker::new(window)
        .background_color(conrod::color::rgb(0.0, 0.3, 0.1))
        .top_left_with_margins_on(CANVAS, 0.0, 15.0)
        .w_h(100.0, 100.0)
        .react(|| println!("Click"))
        .set(COLOR_PICKER, ui);


}

widget_ids! {
    CANVAS,
    COLOR_PICKER,
}