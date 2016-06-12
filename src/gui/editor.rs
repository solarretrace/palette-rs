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
use address::{PageCount, Selection};
// use Color;

use conrod::{
	color,
    Labelable,
    Button,
    Canvas,
    Text,
    Color,
    Colorable,
    Frameable,
    Positionable,
    Sizeable,
    Toggle,
    Widget,
    WidgetMatrix,
};
use conrod;
use piston_window;

use std::sync::mpsc;


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
	/// The current page.
	pub page: PageCount,
	/// The current selection.
	pub selection: Selection,
    /// A channel for sending results to the `WidgetMatrix`.
    pub elem_sender: mpsc::Sender<(usize, usize, bool)>,
    /// A channel for receiving results from the `WidgetMatrix`.
    pub elem_receiver: mpsc::Receiver<(usize, usize, bool)>,
}

impl Editor {
	/// Constructs a new `Editor` for the given palette.
	pub fn new(pal: Palette) -> Self {
		let (elem_sender, elem_receiver) = mpsc::channel();
		Editor {
			frame_width: 1.0,
			palette: pal,
			page: 0,
			selection: Default::default(),
			elem_sender: elem_sender,
			elem_receiver: elem_receiver,
		}
	}
}

/// Set all `Widget`s in the user interface.
pub fn set_widgets(ui: &mut UiCell, editor: &mut Editor) {
	// Root canvas.
    Canvas::new()
        .frame(1.0)
        .pad(30.0)
        .color(color::rgb(100.0, 100.0, 100.0))
        .scroll_kids()
        .set(CANVAS, ui);

        // Text example.
    Text::new("Widget Demonstration")
        .top_left_with_margins_on(CANVAS, 0.0, 20.0)
        .font_size(32)
        .color(color::rgb(0.2, 0.35, 0.45))
        .set(TITLE, ui);


    Button::new()
        .w_h(200.0, 50.0)
        .mid_left_of(CANVAS)
        .down_from(TITLE, 45.0)
        .rgb(0.4, 0.75, 0.6)
        .frame(1.0)
        .label("PRESS")
        .react(|| {println!("pressed");})
        .set(BUTTON, ui)
}

widget_ids! {
    CANVAS,
    TITLE,
    BUTTON,
}