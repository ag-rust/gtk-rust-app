use gdk4::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::*;
use std::cell::Cell;

// Generate GTK boilerplate code with the `#[widget]` macro
// See the gtk4-rs book: https://gtk-rs.org/gtk4-rs/stable/latest/book/gobject_subclassing.html
#[widget(gtk::Box)]
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

    // Signals can be specified with the #[signal] macro.
    #[signal]
    pub card_changed: (),
    // Signals can be specified with the #[signal] macro.
    #[signal]
    pub card_clicked: (),

    // Access children in your template by their id
    // See https://gtk-rs.org/gtk4-rs/stable/latest/book/interface_builder.html
    #[template_child]
    pub card_button: TemplateChild<gtk::Button>,

    #[template_child]
    pub card_entry: TemplateChild<gtk::Entry>,

    #[callback]
    pub on_click: (),
}

impl Card {
    // You have to implement this method, otherwise the `#[widget]` macro will fail;
    pub fn constructed(&self) {
        let s = self;
        self.imp()
            .card_entry
            .connect_changed(glib::clone!(@weak s => move |entry| {
                let text = entry.text().to_string();
                s.imp().text.replace(text);
                s.emit_card_changed()
            }));
    }

    pub fn connect_card_clicked(&self, f: impl Fn(&Self) + 'static) {
        self._connect_card_clicked(f);
    }

    pub fn connect_card_changed(&self, f: impl Fn(&Self) + 'static) {
        self._connect_card_changed(f);
    }

    pub fn text(&self) -> String {
        self.property("text")
    }

    pub fn on_click(&self, _: gtk::Button) {
        self.emit_card_clicked();
    }
}
