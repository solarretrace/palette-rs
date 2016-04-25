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
use Color;

use conrod::{
    Canvas,
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
	/// The diplay matrix for the current page.
	pub color_matrix: [[Color; 16]; 16], 
    /// A channel for sending results to the `WidgetMatrix`.
    pub elem_sender: mpsc::Sender<(usize, usize, bool)>,
    /// A channel for receiving results from the `WidgetMatrix`.
    pub elem_receiver: mpsc::Receiver<(usize, usize, bool)>,
}

impl Editor {
	/// Constructs a new `Editor` for the given palette.
	pub fn new(pal: Palette) -> Self {
		let (elem_sender, elem_receiver) = mpsc::channel();
		let mat = [[Color::new(100, 10, 20); 16]; 16];
		Editor {
			frame_width: 1.0,
			palette: pal,
			page: 0,
			selection: Default::default(),
			color_matrix: mat,
			elem_sender: elem_sender,
			elem_receiver: elem_receiver,
		}
	}
}

/// Set all `Widget`s in the user interface.
pub fn set_widgets(ui: &mut UiCell, app: &mut Editor) {
	// Root canvas.
    Canvas::new()
        .frame(2.0)
        .pad(30.0)
        .color(conrod::Color::Rgba(0.0, 0.0, 0.0, 0.0))
        .scroll_kids()
        .set(CANVAS, ui);

    // Color matrix.
    let (cols, rows) = (16, 16);
    WidgetMatrix::new(cols, rows)
        .down(20.0)
        .w_h(520.0, 520.0) // Matrix width and height.
        .each_widget(|_n, col: usize, row: usize| {
            // Set the color.
            let (r, g, b, a) = (
                1.0, 
                1.0,
                1.0,
                1.0
            );
            // Return the widget we want to set in each element position.
            let elem = true;
            let elem_sender = app.elem_sender.clone();
            Toggle::new(elem)
                .rgba(r, g, b, a)
                .frame(app.frame_width)
                .react(move |new_val: bool|
                	elem_sender.send((col, row, new_val)).unwrap()
                )
        })
        .set(COLOR_MATRIX, ui);

    // Receive updates to the matrix from the `WidgetMatrix`.
    while let Ok((col, row, value)) = app.elem_receiver.try_recv() {
        // app.color_matrix[col][row] = value;
    }
}

widget_ids! {
    CANVAS,
    COLOR_MATRIX
}