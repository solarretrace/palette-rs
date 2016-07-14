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
use super::picker::ColorPicker;

use conrod::{
	color,
    // Labelable,
    // Button,
    Canvas,
    // Text,
    // Color,
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


const GRID_ROWS: usize = 16;
const GRID_COLUMNS: usize = 16;

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
    
    /// The currently displayed colors.
    pub bool_matrix: [[bool; GRID_ROWS]; GRID_COLUMNS],

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
            bool_matrix: [[true; GRID_ROWS]; GRID_COLUMNS],
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
        .color(color::rgb(20.0, 0.0, 0.0))
        .scroll_kids()
        .set(CANVAS, ui);

    let (cols, rows) = (GRID_COLUMNS, GRID_ROWS);
    WidgetMatrix::new(cols, rows)
        .top_left_with_margins_on(CANVAS, 0.0, 15.0)
        .w_h(260.0, 260.0) // matrix width and height.
        .each_widget(|_n, col: usize, row: usize| { // called for every matrix elem.

            // Color effect for fun.
            let (r, g, b, a) = (
                0.5 + (col as f32 / cols as f32) / 2.0,
                0.75,
                1.0 - (row as f32 / rows as f32) / 2.0,
                1.0
            );

            // Now return the widget we want to set in each element position.
            // You can return any type that implements `Widget`.
            // The returned widget will automatically be positioned and sized to the matrix
            // element's rectangle.
            let elem = editor.bool_matrix[col][row];
            let elem_sender = editor.elem_sender.clone();
            Toggle::new(elem)
                .rgba(r, g, b, a)
                .frame(editor.frame_width)
                .react(move |new_val: bool| elem_sender.send((col, row, new_val)).unwrap())
        })
        .set(TOGGLE_MATRIX, ui);

    // Receive updates to the matrix from the `WidgetMatrix`.
    while let Ok((col, row, value)) = editor.elem_receiver.try_recv() {
        editor.bool_matrix[col][row] = value;
    }


    ColorPicker::new()
        .background_color(conrod::color::rgb(0.0, 0.3, 0.1))
        .down(20.0)
        .w_h(256.0, 256.0)
        // This is called when the user clicks the button.
        .react(|| println!("Click"))
        // Add the widget to the conrod::Ui. This schedules the widget it to be
        // drawn when we call Ui::draw.
        .set(COLOR_PICKER, ui);
}

widget_ids! {
    CANVAS,
    TOGGLE_MATRIX,
    COLOR_PICKER,
}