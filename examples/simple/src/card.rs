use gdk4::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::*;
use std::cell::Cell;

// Generate GTK boilerplate code with the `#[widget]` macro
// See the gtk4-rs book: https://gtk-rs.org/gtk4-rs/stable/latest/book/gobject_subclassing.html
#[widget(extends gtk::Box)]
// Define the ui template as described here: https://gtk-rs.org/gtk4-rs/stable/latest/book/interface_builder.html
#[template(file = "card.ui")]
pub struct Card {
    // Your widget properties can be specified using the `#[property_*]` macro
    // There are shortcuts like property_string for string, bool, u64, i64, f64.
    #[property_string]
    pub text: Cell<String>,
    #[property_u64]
    pub num: Cell<u64>,
    // properties can also be specified by providing a ParamSpec
    #[property(glib::ParamSpecBoolean::new(
        "test-prop",
        "",
        "",
        false,
        glib::ParamFlags::READWRITE
    ))]
    test_prop: Cell<bool>,

    // Signals for your custom component can be specified with the #[signal] macro.
    #[signal]
    pub card_changed: (),
    #[signal]
    pub card_clicked: (),

    // Access children in your template by their id
    // See https://gtk-rs.org/gtk4-rs/stable/latest/book/interface_builder.html
    #[template_child]
    pub card_button: TemplateChild<gtk::Button>,
    #[signal_handler(card_button clicked)]
    pub on_card_button_clicked: (),

    #[template_child]
    pub card_entry: TemplateChild<gtk::Entry>,
    #[signal_handler(card_entry changed)]
    pub on_card_entry_changed: (),
}

impl Card {
    // You have to implement this method, otherwise the `#[widget]` macro will fail;
    pub fn constructed(&self) {}

    fn on_card_button_clicked(&self, _b: gtk::Button) {
        self.emit_card_clicked();
    }

    fn on_card_entry_changed(&self, e: gtk::Entry) {
        let text = e.text();
        // You can also access template children via generated accessor methods:
        // let text = self.card_entry().text

        self.set_text(text.into());
        self.emit_card_changed();
    }

    // #[signal] macros generate internal connectors
    // You can choose to define public connectors like this
    pub fn connect_card_clicked(&self, f: impl Fn(&Self) + 'static) {
        self._connect_card_clicked(f);
    }

    pub fn connect_card_changed(&self, f: impl Fn(&Self) + 'static) {
        self._connect_card_changed(f);
    }

    pub fn text(&self) -> String {
        self.property("text")
    }

    fn set_text(&self, text: String) {
        self.set_property("text", text);
    }
}
