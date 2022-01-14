// SPDX-License-Identifier: GPL-3.0-or-later

#[macro_use]
extern crate gtk_rust_app;
#[macro_use]
extern crate log;

use gettextrs::gettext;
use gtk::prelude::*;

// This module will contain our home page
mod home;

fn main() {
    env_logger::init();

    info!("{}", gettext("Check po/ dir for translations."));

    // call app builder with metadata from your Cargo.toml and the gresource file compiled by the `gtk_rust_app::build` script (see below).
    gtk_rust_app::builder::builder(
        include_bytes!("../Cargo.toml"),
        include_bytes!("../target/gra-gen/compiled.gresource"),
    )
    // include your style sheets here
    .styles(include_str!("styles.css"))
    .build(
        |application, _project_descriptor, settings| {
            // Define all navigatable pages of the app
            let pages = vec![gtk_rust_app::components::Page::new(
                home::home(),
                "home",
                Some((gettext("Home"), "go-home-symbolic".into())),
            )];

            // The pages will be placed in this predefined adaptive layout.
            let leaflet_layout =
                gtk_rust_app::components::leaflet_layout(settings, Vec::new(), Vec::new(), pages);
            // and we use the leaflet layout as root content in the apps window.
            let window = gtk_rust_app::window(
                application,
                gettext("Example"),
                settings,
                leaflet_layout.leaflet.upcast_ref(),
            );
            window.show();
        },
        |app, _project_descriptor, _settings| {
            if let Some(action) = app.lookup_action("quit") {
                let simple_action: gdk4::gio::SimpleAction = action.downcast().unwrap();
                simple_action.connect_activate(glib::clone!(@weak app => move |_, _| {
                    app.quit();
                }));
            }
        },
    );
}
