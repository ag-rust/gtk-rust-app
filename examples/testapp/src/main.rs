#[macro_use]
extern crate log;

use gdk4::prelude::{ApplicationExt, ApplicationExtManual};
use gettextrs::gettext;
use gtk::prelude::WidgetExt;
use libadwaita as adw;

fn main() {
    gtk_app_framework::builder()
        .styles(include_bytes!("../assets/styles.css"))
        .view(
            ui::view::home(),
            Some("Home"),
            "home",
            Some("go-home-symbolic"),
        )
        .view(
            ui::view::view2(),
            Some("View 2"),
            "view 2",
            Some("system-switch-user-symbolic"),
        )
        .view(
            ui::view::view3(),
            Some("View 3"),
            "view 3",
            Some("keyboard-layout-filled-symbolic"),
        )
        .build();
}
