// SPDX-License-Identifier: GPL-3.0-or-later

use adw::traits::{ActionRowExt, PreferencesRowExt};
use gdk4::subclass::prelude::ObjectSubclassIsExt;
use gtk::{prelude::*, TemplateChild};
use gtk_rust_app_derive::widget;
use libadwaita as adw;

#[widget(extends gtk::Box)]
#[template(file = "gstore_debug.ui")]
pub struct GstoreDebug {
    #[template_child]
    pub actions_list: TemplateChild<gtk::ListBox>,
    #[template_child]
    pub filter_entry: TemplateChild<gtk::Entry>,
    #[template_child]
    pub scrolled_window: TemplateChild<gtk::ScrolledWindow>,

    #[template_child]
    pub state_text: TemplateChild<gtk::TextView>,

    #[signal_handler(filter_entry changed)]
    pub on_filter: (),
}

impl GstoreDebug {
    pub fn new(recv: Option<glib::Receiver<(gstore::Action, String)>>) -> Self {
        let _self: Self = glib::Object::new(&[]).expect("Failed to create GstoreDebug");
        if let Some(recv) = recv {
            recv.attach(
                None,
                glib::clone!(@weak _self => @default-return glib::Continue(true), move |(a, s)| {
                    _self.on_action(a, s);
                    glib::Continue(true)
                }),
            );
        }
        _self
    }

    pub fn constructed(&self) {}

    fn on_filter(&self, _: gtk::Entry) {
        self.filter();
    }

    fn filter(&self) {
        let search = self.filter_entry().text().to_string();
        if search.is_empty() {
            let mut c = self.actions_list().first_child();
            while let Some(child) = &c {
                child.set_visible(true);
                c = child.next_sibling();
            }
            return;
        }
        let searches: Vec<&str> = search.split(',').collect();
        let mut c = self.actions_list().first_child();
        while let Some(child) = &c {
            if let Some(row) = child.dynamic_cast_ref::<adw::ActionRow>() {
                let title = row.title().to_string();
                let visible = searches.iter().any(|x| {
                    if let Some(t) = x.strip_prefix('!') {
                        title != t
                    } else {
                        title.starts_with(x)
                    }
                });
                row.set_visible(visible);
            }
            c = child.next_sibling();
        }
    }

    fn on_action(&self, action: gstore::Action, state: String) {
        let mut row_builder = adw::ActionRow::builder().title(action.name());
        if let Some(d) = glib::DateTime::now(&glib::TimeZone::utc())
            .ok()
            .and_then(|d| d.format_iso8601().ok())
        {
            row_builder = row_builder.subtitle(&d);
        }
        let row = row_builder.build();
        if let Some(v) = action.argument() {
            let label = gtk::Label::new(Some(&format!("{}", v)));
            label.set_ellipsize(gdk4::pango::EllipsizeMode::End);
            row.add_suffix(&label);
        }

        let scroll_y = self.scrolled_window().vadjustment().value();
        let row_height = 50.0;

        self.actions_list().prepend(&row);

        self.filter();

        if scroll_y > 0.0 {
            self.scrolled_window()
                .vadjustment()
                .set_value(scroll_y + row_height);
        }

        let buf = self.state_text().buffer();
        buf.set_text(&state);
    }
}

impl Default for GstoreDebug {
    fn default() -> Self {
        Self::new(None)
    }
}
