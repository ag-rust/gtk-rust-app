# gtk-rust-app documentation

gtk-rust-app is a framework that enables you to write native linux apps in Rust using GTK in a functional style and fill the gap between Rust programming and GTKs object oriented approach.

It does that by providing some out-of-the-box components that implement common boilerplate code and helps to focus on the domain UI development.

## Functional style

Similar to react, a part of **UI code** (may be called a component) is just a **function** that describes the UI part.

Such a component may look like this.

```rust
pub use gtk-rust-app::interface;
pub use gtk::prelude::*;

pub fn component() -> gtk::Widget {
    interface!(r#"
        <object class="GtkBox" id="comp">
            <property name="orientation">vertical</object>
            
            <child>
            <object class="GtkLabel" id="lbl">
                <property name="label">Hello</property>
            </object>
            </child>
            
            <child>
            <object class="GtkButton" id="btn">
                <property name="label">World</property>
            </object>
            </child>

        </object>
    "#
        comp: gtk::Box,
        lbl: gtk::Label,
        btn: gtk::Button
    )

    comp.upcast()
}
```

The `interface` macro allows to define the user interface with the usual GTK UI markup language while all GTK Widget can be obtained as variables to do anything with them.

## Domain model and GObjects

GTK is object oriented, Rust is not but we still need to have GObjects sometimes. An Example:

We have a domain struct `TodoItem`. Our application state stores these `TodoItems` in a vec and we want to select a single one in a GTK combobox (Or more likely in a `AdwComboRow`) menu. GTK expects a combobox to have a backing `model` which is a list of `GObjects`. Writing a GObject for our `TodoItem` is a lot of boilerplate code and we might not need the whole *objectiveness* because we are not writing object oriented code. Nevertheless we want our combobox to show the possible `TodoItems` and select one probably knowing the selected Id.

To address this problem `gtk-rust-app` provides the attribute macro `gobjectify`. The macro allows to define a set of fields for a struct which will be used to generate a GObject definition.

```rust
#[gobjectify(id, name)]
struct TodoItem {
    id: String,
    name: String,
    text: String,
}
```

Will generate the GObject `TodoItemGObject` with the properties `id` and `name` and a public method `TodoItem.gobjectify() -> TodoItemGObject`.

