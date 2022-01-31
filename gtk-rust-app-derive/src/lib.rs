// SPDX-License-Identifier: GPL-3.0-or-later

extern crate proc_macro;

use proc_macro::TokenStream;

mod gobject;
mod variant;
mod widget;

/// Define GObject based on a struct
///
/// # Example
/// ```rust,ignore
/// #[gobject(id, name, selected)]
/// struct Event {
///     id: String,
///     name: String,
///     selected: bool,
/// }
/// ```
///
/// ## Supported field types
/// - String
/// - bool
#[proc_macro_attribute]
pub fn gobject(args: TokenStream, input: TokenStream) -> TokenStream {
    gobject::gobject(args, input)
}

/// Define a new GTK widget based on a struct
/// # Example
/// ```rust,ignore
/// use gdk4::subclass::prelude::ObjectSubclassIsExt;
/// use glib::closure_local;
/// use gtk::prelude::*;
/// use std::cell::Cell;
///
/// #[widget]
/// #[template(file = "card.ui")]
/// pub struct Card {
///     #[property_string]
///     pub text: Cell<String>,
///     #[signal]
///     pub card_changed: (),
///     #[signal]
///     pub card_clicked: (),
///
///     #[template_child]
///     pub card_button: TemplateChild<gtk::Button>,
///
///     #[template_child]
///     pub card_entry: TemplateChild<gtk::Entry>,
/// }
///
/// impl Card {
///     pub fn new(&self) {
///         let s = self;
///         self.imp()
///             .card_button
///             .connect_clicked(glib::clone!(@weak s => move |_| {
///                 s.emit_card_clicked()
///             }));
///         self.imp()
///             .card_entry
///             .connect_changed(glib::clone!(@weak s => move |entry| {
///                 let text = entry.text().to_string();
///                 s.imp().text.replace(text);
///                 s.emit_card_changed()
///             }));
///     }
///
///     // If you want to have a public connector you can define one like this.
///     // The method _connect_<signal-name> is generated for this purpose
///     pub fn conenct_card_clicked(&self, f: impl Fn(&Self) + 'static) {
///         self._connect_card_clicked(f);
///     }
///
///     pub fn connect_card_changed(&self, f: impl Fn(&Self) + 'static) {
///         self._connect_card_changed(f);
///     }
///
///     pub fn text(&self) -> String {
///         self.property("text")
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn widget(args: TokenStream, input: TokenStream) -> TokenStream {
    widget::widget(args, input)
}

/// Auto implement FromVariant and ToVariant for a struct
#[proc_macro_attribute]
pub fn variant_serde_json(args: TokenStream, input: TokenStream) -> TokenStream {
    variant::variant_serde_json(args, input)
}
