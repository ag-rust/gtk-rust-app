// SPDX-License-Identifier: GPL-3.0-or-later

use gdk4::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::*;
use gettextrs::gettext;
use crate::card::Card;

// Define a page of your app as a new widget
#[widget(gtk::Box)]
#[template(file = "home.ui")]
struct Home {
    #[template_child]
    pub card: TemplateChild<Card>,
}

impl Home {
    pub fn constructed(&self) {
        self.imp().card.connect_card_clicked(|card| {
            println!("Text prop: {:?}", card.text());
        });
    }

    pub fn new() -> Home {
        glib::Object::new(&[]).expect("Failed to create Home")
    }
}

impl gtk_rust_app::widgets::Page for Home {
    fn name(&self) -> &'static str {
        "home"
    }

    fn title_and_icon(&self) -> Option<(String, String)> {
        Some((gettext("Home"), "go-home-symbolic".into()))
    }
}
