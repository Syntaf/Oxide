# Oxide: rustty with widgets

An easy-to-use and extensible GUI library for Rust terminal applications.

## Intro

Oxide is an add-on extension for [rustty](https://github.com/cpjreynolds/rustty) that provides a widget based API 
for building terminal UI applications. The project is heavily inspiried by Pythons TKinter, and aims at simplicity
and modularity.

See [GameOfLife](https://github.com/Syntaf/GameOfLife) for a good example of what Oxide is capable of, and feel free
to post issues if you run into trouble using Oxide.

For documentation on using rustty, see https://github.com/cpjreynolds/rustty

## Design Philosophy

Oxide is built on the philosophy that all GUI elements begin as a _Widget_, and can later be specialized for certain task. By
providing a common ancestor for ALL UI components, Oxide can generalize the components interact with each other and create
an easily extensible interface for developers. To visualize this relationship, see fig 1 below



There are a couple of key traits in this redesign currently: Widget, Button, and Layout.

Widgets are the core of the API, all frontend structs offered to the user inherit from Widget. This generalizes the way structs interact with each other and makes encapsulation and specialized widgets easier to develop. Button and Layout are specialized Widgets that offer new functionality that is distinguishable.

Frontend structs all offer a similar API (see Implementation section for a list). This makes developing new widgets easier and interactions less buggy (we don't need to worry about a specific widget interacting with another specific widget, because they are generalized by their traits).

### Widgets


fe has a couple key concepts: generalized widgets and user customization. The design of the widgets are inspired by 
Tkinter and aims to have a similar form and function. fe supplies a basic number of widgets that are useful for 
designing application, but you can easily write your own widgets using the traits available.

To get started with widgets, your most basic and most widely used container will be a `Dialog`. A dialog's job is to
mesh widgets together and act as a aggregator, a widget which *takes in* other widgets for easier management. Creation
of a dialog is simple:

```rust
let mut dlg = Dialog::new(60, 10);	// create dialog 60 columns wide, 10 rows long
dlg.draw_box();				// draw border
```

This dialog will now allow us to aggregate widgets we wish to bundle together, say labels or buttons:


```rust
let mut b1 = StdButton::new("Quit", 'q', ButtonResult::Ok);		// Create button
b1.pack(&maindlg, HorizontalAlign::Left, VerticalAlign::Bottom, (4,2));	// Align button within dialog

dlg.add_button(b1);	// dlg now takes ownership of b1
```

Great! now when we want to poll events, we can use dialogs to forward events to our buttons

```rust
// Poll events
while let Some(Event::Key(ch)) = terminal.get_event(0).unwrap() {
    match dlg.result_for_key(ch) {
        Some(ButtonResult::Ok) => break 'main,
	_ => {},
    }
}
```

Widgets can still function as independent objects, but Dialogs helps bring everything together and let you
encapsulate your data for better abstraction.  

A good way to understand fe widgets are that they are simply specialized frames that *own* an area of cells, 
and perform actions based on that specliazation. At their core, widgets implement a frame and some basic trait 
specialization. Take for example a label:


```rust
pub struct Label {
    frame: Frame,
    text: Vec<String>
    // ...
}
```

Our widget in this case has a frame, and uses `text` for drawing into that frame. `Frames` simply represent an area 
that a widget owns. Multiple widgets can own the same cells, but each widget is only concered with itself. In the
case of a Label, we want to write text to an area of cells. Like other widgets, this Label can be owned by a dialog,
packed, and drawn to the screen. 

Any widget implements basic functionality: drawing, packing, outlining, resizing, and returning the *frame*. In
most cases the actual widget is the frame, and structs like `Label` or `Button` *wrap* a frame to provide special
functionality.

## Usage Guide

Examples and usage suggestions can be found in the [API
documentation][1].

## Contact

[email me](mailto:syntaf@gmail.com)

[1]: http://syntaf.github.io/ruik
