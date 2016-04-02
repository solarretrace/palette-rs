

Building a GUI application in Rust.
===================================

Our goal is to build a GUI around an existing rust application. This tutorial will be an unscripted journey toward making that happen, and it will be as long as it needs to be. So I suppose it will also be a kind of journal where I keep track of everything I'm trying, why I'm trying it, and what my thought process is throughout. Hopefully it will be useful to you. Or at least provide some valuable, if incidental, insight.

## Motivation

I've built a couple of applications in Rust already: a graph traversal library, a Pratt Parser builder, and more recently, a structured color palette editor. Unfortunately, these are all command line tools, which specifically causes problems for the users of the palette editor: I can't easily display colors in the terminal. So naturally, I'm looking for a good (hopefully cross-platform) way to build a GUI wrapper for manipulating my palette editor.

Furthermore, I have a bit of an interest in game design, and have spent some time building a 3D game engine in my free time, so I'd like to learn something that intersects well with that.

## Piston 

Piston is a game engine written in Rust and designed to be a pluggable, platform-agnostic library. Looking at the repository sources, it appears that the 'Piston' library is really just a set of traits that provide an interface to the supported libraries. Thus to use Piston, you'll have to also grab these libraries (which are conveniently listed [here]()). To do this, you only have to include them as dependencies in cargo.

## Conrod

Conrod is a fledgeling GUI library built on top of Piston. Since my palette editor is fairly simple, and I don't imagine it will need to employ many standard widgets, I am ok giving it a pared-down look or building my own custom widgets. Being dependent upon Piston, it may also provide some insight into using Rust for creating games. 

If this doesn't work out for some reason, I would next look into finding a rust gtk wrapper. And if that was unsuitable, I would try to use emscripten and build a browser-based GUI, though that seems like a ton of work. (Especially since I'm not an exprienced web programmer.) Lastly, I would try to build my GUI in another language and use FFI to interact with my rust code.

# First steps.

I followed the directions on the Conrod repo, which had me clone the repo and use cargo to build it. This went fairly quickly, and aside from some depricated lints, there were no errors.

Next, I ran the provided examples to make sure everything works and to see what I'm actually working with here.

There's some strange behavior in the text boxes on OS X after typing some unicode characters. It seems to start inserting boxes whenever I press the arrow keys. This is probably a bug...


Reading the Documentation
===========================

There isn't any documentation... :)  Well, that's a lie, but it's a good first-order approximation to the truth, So lets read some of the examples instead. 

Looking at [all_widgets.rd](https://github.com/PistonDevelopers/conrod/blob/master/examples/all_widgets.rs), The code is broken into a number of different sections:

  + Imports
  + Type aliases
  + GUI state struct
  + main()
  + set_widgets()
  + Widget ID generation

## Imports

```rust
#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;

use conrod::{
    color,
    Button,
    Canvas,
    Circle,
    Color,
    Colorable,
    DropDownList,
    EnvelopeEditor,
    Frameable,
    Labelable,
    NumberDialer,
    Point,
    Positionable,
    Slider,
    Sizeable,
    Text,
    TextBox,
    Theme,
    Toggle,
    Widget,
    WidgetMatrix,
    XYPad,
};
use piston_window::{EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings};
use std::sync::mpsc;
```

The `extern crate conrod` line should let our program use conrod's functionality, and the `#[macro_use]` attribute will let us use the `widget_ids!` macro that we'll use later. The `find_folder` crate is being used to locate an assets directory for the UI font. 

The `piston_window` crate defines a basic window configuration. This looks to be part of the configurable backend that piston advertises. The [Piston] crate itself seems to only provide an interface, while crates like `piston_window` provide implementations.

The [`std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/) module is providing channels for use in sending updates through a widget matrix. I'll look into how this works does later...

## Type aliases

```rust
/// Conrod is backend agnostic. Here, we define the `piston_window` backend to use for our `Ui`.
type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;
```

This looks pretty straightforward. Just have to keep these in mind when reading forward.

## GUI state struct

```rust
struct DemoApp {
    bg_color: Color,
    show_button: bool,
    toggle_label: String,
    title_pad: f64,
    v_slider_height: f64,
    frame_width: f64,
    bool_matrix: [[bool; 8]; 8],
    ddl_colors: Vec<String>,
    ddl_color: Color,
    selected_idx: Option<usize>,
    circle_pos: Point,
    envelopes: Vec<(Vec<Point>, String)>,
    elem_sender: mpsc::Sender<(usize, usize, bool)>,
    elem_receiver: mpsc::Receiver<(usize, usize, bool)>,
}

impl DemoApp {
    fn new() -> DemoApp {
        let (elem_sender, elem_receiver) = mpsc::channel();
        DemoApp {
            bg_color: color::rgb(0.2, 0.35, 0.45),
            show_button: false,
            toggle_label: "OFF".to_string(),
            title_pad: 350.0,
            v_slider_height: 230.0,
            frame_width: 1.0,
            bool_matrix: [ [true, true, true, true, true, true, true, true],
                           [true, false, false, false, false, false, false, true],
                           [true, false, true, false, true, true, true, true],
                           [true, false, true, false, true, true, true, true],
                           [true, false, false, false, true, true, true, true],
                           [true, true, true, true, true, true, true, true],
                           [true, true, false, true, false, false, false, true],
                           [true, true, true, true, true, true, true, true] ],
            ddl_colors: vec!["Black".to_string(),
                              "White".to_string(),
                              "Red".to_string(),
                              "Green".to_string(),
                              "Blue".to_string()],
            ddl_color: color::PURPLE,
            selected_idx: None,
            circle_pos: [-50.0, 110.0],
            envelopes: vec![(vec![ [0.0, 0.0],
                                   [0.1, 17000.0],
                                   [0.25, 8000.0],
                                   [0.5, 2000.0],
                                   [1.0, 0.0], ], "Envelope A".to_string()),
                            (vec![ [0.0, 0.85],
                                   [0.3, 0.2],
                                   [0.6, 0.6],
                                   [1.0, 0.0], ], "Envelope B".to_string())],
            elem_sender: elem_sender,
            elem_receiver: elem_receiver,
        }
    }
}
```

This is a single struct that represents the state modeled by our GUI. Only the dynamic parts of the GUI need to be here, as far as I can tell. They are given default values in the constructor. Note also that we have the `mpsc` channel components here. These are also simply instantiated anew when this object is created.

## main()

```rust
fn main() {
    let window: PistonWindow =
        WindowSettings::new("All The Widgets!", [1100, 560])
            .exit_on_esc(true).vsync(true).build().unwrap();

    let mut ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    let mut app = DemoApp::new();

    for event in window.ups(60) {
        ui.handle_event(&event);
        event.update(|_| ui.set_widgets(|mut ui| set_widgets(&mut ui, &mut app)));
        event.draw_2d(|c, g| ui.draw_if_changed(c, g));
    }
}
```

We start with some setup:
  + Create a new window. 
  + Create the UI (a `conrad::Ui<Backend>`) and set the assets, font, theme, and glyph cache. 
  + Create a new, default state for the GUI.

Next, we run a loop for handling the window events, updating the GUI state, and drawing the GUI. We've got a fairly confusing closure here that involves calling `ui.set_widgets`, which takes a closure providing the `Ui` itself, upon which we'll call our own global `set_widgets` function, passing it that same `Ui` object. That's some kind of indirection. It seems like the whole point of this is to introduce a dependency upon `app`, our widget state object, so that we can define a single function which can see both `ui` and `app` in order to create our widgets based on the current state of the program.

All of this is happening in the `event.update` call, which is just taking a closure to act as a callback. The `event.draw_2d` function does the same thing, and we can choose to either draw it every frame with `ui.draw`, or only draw the changed widgets with `ui.draw_if_changed`.

## set_widgets()
```rust
fn set_widgets(ui: &mut UiCell, app: &mut DemoApp) { /* ... */ }
```

Here we are defining what our widgets will look like given the state of our GUI. This function is quite long, so I'll just cover the interesting parts.

```rust
Canvas::new()
    .frame(app.frame_width)
    .pad(30.0)
    .color(app.bg_color)
    .scroll_kids()
    .set(CANVAS, ui);
```

This is a canvas widget that other widgets will be children of. There's no positioning on here, so I assume the default for a canvas is to cover the whole space. The `set` method seems to be out builder pattern's "build" method, which assigns an ID `CANVAS` to this object and as well as associating it with the given `UiCell`. 

Now, I was calling this a `Ui` above, because in the `ui.set_widgets` closure argument, it was just called `ui`, but it seems that this closure is actually being passed a `UiCell`, which I imagine is just some kind of view of the `Ui` under the associated `Backend`. It's not clear what this relationship is, but it seems like it won't cause any obstacles from here.

```rust
Text::new("Widget Demonstration")
    .top_left_with_margins_on(CANVAS, 0.0, app.title_pad)
    .font_size(32)
    .color(app.bg_color.plain_contrast())
    .set(TITLE, ui);
```

This is our first widget. The `top_left_with_margins_on` method it telling this widget to be overlapping the given widget. We see other widgets using `..._of` methods, which presumably means to place the widgets next to eachother. Here', we're placing it in the top left of the canvas using the given arguments as margin settings. This method is provided by the [`Positionable`](http://docs.piston.rs/conrod/conrod/trait.Positionable.html) trait, so we can see what it needs from there:

```rust
fn top_left_with_margins_on<I>(self, other: I, top: Scalar, left: Scalar) -> Self where I: Into<Index> + Copy { ... }
```

So `0.0` is the width of the top margin, and `app.title_pad` is the width of the left margin.

Interactive widgets all seem to have a `react` method which takes a closure that defines the action to take when the widget is used. For instance, in the button we have `.react(|| app.bg_color = color::random())`, and in the slider we have `.react(|new_pad: f32| app.title_pad = new_pad as f64)`. On the envelope text boxes, we have a conditional builder method:

```rust
.and_if(i == 0, |text| text.right_from(COLOR_SELECT, 30.0))
```

Most of the rest of the builder methods seem quite obvious, except for maybe the `each_widget` method on the WidgetMatrix. This function takes a closure which provides the position of the required widget and returns the widget that goes in that position. This is where the `mpsc` channels come into play. Each of the generated widgets has a cloned copy of `elem_sender`, which it uses to send its identifying information and state through: Then, in the `set_widgets` function, we use the `elem_receiver` to try and receive updates from these elements. If we get an update, then we know which widget needs to have its state changed.

A simplified version of the code looks like this:

```rust
let (cols, rows) = (8, 8);
WidgetMatrix::new(cols, rows)
    .each_widget(|_n, col: usize, row: usize| {
        let elem = app.bool_matrix[col][row];
        let elem_sender = app.elem_sender.clone(); // Clone the sender.
        Toggle::new(elem)
            .rgba(r, g, b, a)
            .frame(app.frame_width)
            .react(move |new_val: bool| elem_sender.send((col, row, new_val)) // Send messages when interacted with.
            .unwrap())
    })
    .set(TOGGLE_MATRIX, ui);

/// Receive messages and update the state.
while let Ok((col, row, value)) = app.elem_receiver.try_recv() {
    app.bool_matrix[col][row] = value;
}
```

Notice that we don't call `set` on the individual `Toggle`s. Presumably, the WidgetMatrix will own these objects and track them under its own ID.

## Widget ID generation

```rust
widget_ids! {
    CANVAS,
    TITLE,
    BUTTON,
    TITLE_PAD_SLIDER,
    TOGGLE,
    COLOR_SLIDER with 3,
    SLIDER_HEIGHT,
    FRAME_WIDTH,
    TOGGLE_MATRIX,
    COLOR_SELECT,
    CIRCLE_POSITION,
    CIRCLE,
    ENVELOPE_EDITOR with 4
}
```

The `widget_ids!` macro is generating unique IDs for each of our widgets. The `with n` syntax tells it to generate `n` unique IDs. The purpose of these IDs it to track which widgets have changed on each frame. From what I've read, this prevents the widgets from being recreated every frame, though by the looks of the code, it seems this is an all-or-nothing proposition: if any of your widgets change, then they will all be recreated and updated. This makes sense because they widgets can update eachother, but it could be made more efficient if it only updated the ones dependent upon the changed one. However, this would make the interface quite a bit more clunky, because you'd have to have different update functions for each widget.

With this analysis under our belts, we should be able to dive in and create a GUI for our existing program.

The Palette Editor
==================

The palette editor is not your average 'collection of colors' editor. This is a *structured* palette editor, which means that colors are best defined by their relationships to other colors. Most images have color gradients. Rather than flatly specifying each color in the gradient, we want to specify the boundary colors of the gradient and an interpolation function. Furthermore, we may with to apply blending and other transformations to the gradient. Our palette should be able to talk about colors in these terms, and allow us to iterate over the requested colors which will be generated lazily upon request.

Additionally, we'll need some way to arrange these colors in a user-friendly manner. Lets say you have a couple of different connected color structures, and you use these to represent different properties of your image. For example, you may have a 'wood color' ramp and a 'water color' ramp. We want to take all the colors associated with one ramp and seperate them from the other somehow. To do this, I'm using a 'Page/Line/Column' addressing scheme. This gives us a view of our colors such that we can look at a whole page of colors at once, with each page consisting of ramps in lines, and each color occupying a single column. This will make it more convenient to arrange and label colors, as well as applying operations to multiple colors at once.

Lets go over some example code. Here's a very simple structured palette being created in a rust program:

```rust
extern crate rampeditor;

use rampeditor::*;

fn main() {
    let mut pal = ZplPalette::new("Test Palette");
    
    pal.apply(
        InsertColor::new(Color(50, 50, 78))
            .located_at(Address::new(0, 0, 0))
    ).ok();

    pal.apply(
        InsertColor::new(Color(0, 0, 255))
            .located_at(Address::new(0, 0, 1))
    ).ok();

    pal.apply(
        InsertRamp::new(Address::new(0, 0, 0), Address::new(0, 0, 1), 6)
            .located_at(Address::new(0, 1, 0))
    ).ok();

    println!("{}", pal);
}

```

First, we create a new palette. (The configuration of the palette is set by the palette type. This allows us to easily convert palettes into different formats for exporting and preventing us from doing things prohibbeted by that format.)

Second, we apply some ColorElement generating operations to the palette. We insert two new colors and place them at the given addresses. Third, we generate a basic ramp of 6 colors that will interpolate linearly between the given colors in rgb space. The output of this program is as follows:

```
ZplPalette 1.0.0 [History: 3 items]
"Test Palette" (ZplPalette 1.0.0) [Lines: 0] [Columns: 0] [515 pages] [8 elements] [default wrap 16:16]
Page 0:*:* - "Main" (Level 0) [Lines: 14] [Columns: 0]
    (Main CSET 0) [Lines: 0] [Columns: 16]
    Address   Color    Order
    00:00:00  #32324E  0
    00:00:01  #0000FF  0
    (Main CSET 1) [Lines: 0] [Columns: 16]
    Address   Color    Order
    00:01:00  #2A2A67  2
    00:01:01  #232380  2
    00:01:02  #1C1C99  2
    00:01:03  #1515B3  2
    00:01:04  #0E0ECC  2
    00:01:05  #0707E5  2
```

There are a few things to notice here. First, most of this stuff is set by the palette format, which provides utilities like auto-wrapping after a certain line length, default line/page names, or an undo history. (This is why the `pal.apply` functions were invoked passing in `PaletteOperation`s, rather that something simpler like `pal.insert_color`. The 'order' of a color expresses the number of dependencies it has. Raw colors have 0 order, while the linear ramp between two colors has order 2.

By adding some more code to the end of our main function, we can see the beauty of a structured palette editor. Let's add the following code just before the `println!` statement:

```rust
    pal.apply(
        InsertColor::new(Color(0, 100, 100))
            .located_at(Address::new(0, 0, 0))
            .overwrite(true)
    ).ok();
```

This produces the following output:

```
ZplPalette 1.0.0 [History: 4 items]
"Test Palette" (ZplPalette 1.0.0) [Lines: 0] [Columns: 0] [515 pages] [8 elements] [default wrap 16:16]
Page 0:*:* - "Main" (Level 0) [Lines: 14] [Columns: 0]
    (Main CSET 0) [Lines: 0] [Columns: 16]
    Address   Color    Order
    00:00:00  #006464  0
    00:00:01  #0000FF  0
    (Main CSET 1) [Lines: 0] [Columns: 16]
    Address   Color    Order
    00:01:00  #00557A  2
    00:01:01  #004790  2
    00:01:02  #0039A6  2
    00:01:03  #002ABC  2
    00:01:04  #001CD2  2
    00:01:05  #000EE8  2
```

Notice how all the ramp colors (which were dependent upon the inserted colors) change in responce to our overwriting of an existing color. In this way, our two original colors act as controls for the ramp. We could, if we wished, add additional ramps that depend upon other ramps. The only thing we can't do is add a dependency loop to the palette.

## Onward

With that said, we want to build a GUI for this thing! We need to display the colors in the palette. Switch our view to different pages. Provide a means for arranging colors, as well as selecting and modifying them. And we need to provide an interface for all the operations defined in the palette operation modules. (Which are only partially implemented at this point.) We also need color selection and file opening and saving features at some point.