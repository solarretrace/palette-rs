

use conrod::{
    Backend,
    Color,
    Colorable,
    CommonBuilder,
    FontSize,
    Image,
    IndexSlot,
    UpdateArgs,
    Widget,
    WidgetKind,
};

use piston_window::{PistonWindow, Texture};
use piston_window;
use image;
use image::ImageBuffer;

use std::sync::Arc;

/// A `&'static str` that can be used to uniquely identify a `ColorPicker`.
pub const KIND: WidgetKind = "ColorPicker";


/// A widget for picking colors.
pub struct ColorPicker<'w, F> {
    /// Common widget components.
    common: CommonBuilder,
    /// The style configuration for the widget.
    style: Style,
    /// Whether the widget is available for input.
    enabled: bool,
    /// The window in which the widget will be drawn. This is used to manage 
    /// resources associated with the image buffer of the selector.
    window: &'w mut PistonWindow,
    /// Optional callback for when the color selection is changed.
    selection_react: Option<F>,
}

impl<'w, F> ColorPicker<'w, F> {
    /// Create a button context to be built upon.
    pub fn new(window: &mut PistonWindow) -> ColorPicker<F> {
        ColorPicker {
            common: CommonBuilder::new(),
            style: Style::new(),
            enabled: true,
            window: window,
            selection_react: None,
        }
    }

    /// Set the reaction for the ColorPicker. The reaction will be triggered 
    /// when the color selection changes.
    pub fn react(mut self, reaction: F) -> Self {
        self.selection_react = Some(reaction);
        self
    }

    /// Sets whether the ColorPicker accepts input.
    #[allow(dead_code)]
    pub fn enabled(mut self, flag: bool) -> Self {
        self.enabled = flag;
        self
    }

    builder_methods! {
        pub background_color { style.background_color = Some(Color) }
    }
}


// Generate a Style struct with theme defaults
widget_style!{
    KIND;
    /// Represents the styling for the ColorPicker widget.
    style Style {
        /// Color of the background.
        - background_color: Color { theme.background_color }
        /// Font size of the widget's controls.
        - font_size: FontSize { theme.font_size_medium }
    }
}



/// Represents the unique, cached state for our ColorPicker widget.
#[derive(Clone, Debug, PartialEq)]
pub struct State {
    /// An index to use for our **Image** primitive graphics widget.
    image_idx: IndexSlot,

    buf_width: u32,
    buf_height: u32,
}




impl<'w, F> Widget for ColorPicker<'w, F> where F: FnMut() {
    type State = State;
    type Style = Style;

    fn common(&self) -> &CommonBuilder {
        &self.common
    }

    fn common_mut(&mut self) -> &mut CommonBuilder {
        &mut self.common
    }

    fn unique_kind(&self) -> &'static str {
        KIND
    }

    fn init_state(&self) -> State {
        State {
            image_idx: IndexSlot::new(),
            buf_width: 20,
            buf_height: 20,
        }
    }

    fn style(&self) -> Style {
        self.style.clone()
    }

    fn update<B: Backend>(self, args: UpdateArgs<Self, B>) {

        let UpdateArgs {idx, state, rect, mut ui, style, ..} = args;

        
        let buf = ImageBuffer::from_fn(
            state.buf_width, 
            state.buf_height, 
            |x, y| {
                if (x+y) % 2 == 0 {
                    image::Rgba([0u8, 0u8, 0u8, 0u8])
                } else {
                    image::Rgba([255u8, 255u8, 255u8, 255u8])
                }
            }
        );

        let buf_texture = Texture::from_image(
            &mut self.window.factory,
            &buf,
            &piston_window::TextureSettings::new()
        ).unwrap();

        Image::from_texture(Arc::new(buf_texture))
            .set(state.image_idx.get(&mut ui), &mut ui);
    }
}

/// Provide the chainable color() configuration method.
impl<'w, F> Colorable for ColorPicker<'w, F> {
    fn color(mut self, color: Color) -> Self {
        self.style.background_color = Some(color);
        self
    }
}

// impl<'w, F> Sizeable for ColorPicker<'w, F> {
//     fn x_dimension(self, x: Dimension) -> Self;
//     fn y_dimension(self, x: Dimension) -> Self;
//     fn get_x_dimension<B: Backend>(&self, ui: &Ui<B>) -> Dimension;
//     fn get_y_dimension<B: Backend>(&self, ui: &Ui<B>) -> Dimension;
// }

