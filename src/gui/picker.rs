

// use piston_window::Gfx2d;


// fn get_texture() -> Texture {
    // let mut image = image::ImageBuf::new(width, height);
    // let mut draw = false;
    // let mut texture = Texture::from_image(&image);
// }



/*----------------------------------------------------------------------------*/

use conrod::{
    // self,
    Backend,
    // Circle,
    Color,
    Colorable,
    CommonBuilder,
    // Dimensions,
    FontSize,
    IndexSlot,
    // Labelable,
    // Point,
    // Positionable,
    // Text,
    UpdateArgs,
    Widget,
    WidgetKind,
};


/// A `&'static str` that can be used to uniquely identify a `ColorPicker`.
pub const KIND: WidgetKind = "ColorPicker";


/// A widget for picking colors.
pub struct ColorPicker<F> {
    /// Common widget components.
    common: CommonBuilder,
    /// The style configuration for the widget.
    style: Style,
    /// Whether the widget is available for input.
    enabled: bool,
    /// Optional callback for when the color selection is changed.
    selection_react: Option<F>,
}

impl<F> ColorPicker<F> {
    /// Create a button context to be built upon.
    pub fn new() -> ColorPicker<F> {
        ColorPicker {
            common: CommonBuilder::new(),
            style: Style::new(),
            enabled: true,
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
    /// An index to use for our **Circle** primitive graphics widget.
    circle_idx: IndexSlot,
    /// An index to use for our **Text** primitive graphics widget (for the label).
    text_idx: IndexSlot,
}




impl<F> Widget for ColorPicker<F> where F: FnMut() {
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
            circle_idx: IndexSlot::new(),
            text_idx: IndexSlot::new(),
        }
    }

    fn style(&self) -> Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update<B: Backend>(self, args: UpdateArgs<Self, B>) {
        
        // let UpdateArgs { idx, state, rect, mut ui, style, .. } = args;

        // let color = {
        //     let input = ui.widget_input(idx);

        //     // If the button was clicked, call the user's `react` function.
        //     if input.clicks().left().next().is_some() {
        //         if let Some(mut react) = self.maybe_react {
        //             react();
        //         }
        //     }

        //     let color = style.color(ui.theme());
        //     input.mouse()
        //         .map(|mouse| {
        //             if is_over_circ([0.0, 0.0], mouse.rel_xy(), rect.dim()) {
        //                 if mouse.buttons.left().is_down() {
        //                     color.clicked()
        //                 } else {
        //                     color.highlighted()
        //                 }
        //             } else {
        //                 color
        //             }
        //         })
        //         .unwrap_or(color)
        // };

        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        // Conrod will automatically determine whether or not any changes have occurred and
        // whether or not any widgets need to be re-drawn.
        //
        // The primitive graphics widgets are special in that their unique state is used within
        // conrod's backend to do the actual drawing. This allows us to build up more complex
        // widgets by using these simple primitives with our familiar layout, coloring, etc
        // methods.
        //
        // If you notice that conrod is missing some sort of primitive graphics that you
        // require, please file an issue or open a PR so we can add it! :)

        // First, we'll draw the **Circle** with a radius that is half our given width.
        // let radius = rect.w() / 2.0;
        // let circle_idx = state.circle_idx.get(&mut ui);
        // Circle::fill(radius)
        //     .middle_of(idx)
        //     .graphics_for(idx)
        //     .color(color)
        //     .set(circle_idx, &mut ui);

        // // Now we'll instantiate our label using the **Text** widget.
        // let label_color = style.label_color(ui.theme());
        // let font_size = style.label_font_size(ui.theme());
        // let text_idx = state.text_idx.get(&mut ui);
        // if let Some(ref label) = self.maybe_label {
        //     Text::new(label)
        //         .middle_of(idx)
        //         .font_size(font_size)
        //         .graphics_for(idx)
        //         .color(label_color)
        //         .set(text_idx, &mut ui);
        // }
    }

}

/// Provide the chainable color() configuration method.
impl<F> Colorable for ColorPicker<F> {
    fn color(mut self, color: Color) -> Self {
        self.style.background_color = Some(color);
        self
    }
}


