// SPDX-License-Identifier: GPL-3.0-or-later

use gtk::prelude::*;
// use libadwaita as adw;

#[cfg(not(feature = "libadwaita"))]
pub fn window(
    app: &gtk::Application,
    title: String,
    settings: Option<&gdk4::gio::Settings>,
    root: &gtk::Widget,
) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title(&title)
        .child(root)
        .build();

    if let Some(settings) = settings {
        let w = settings.get("window-width");
        let h = settings.get("window-height");

        window.set_default_width(w);
        window.set_default_height(h);

        window.connect_close_request(glib::clone!(@strong settings => move |win| {
            let width = win.width();
            let height = win.height();
            settings.set_int("window-width", width - 122).unwrap();
            settings.set_int("window-height", height - 122).unwrap();
            gtk::Inhibit(false)
        }));
    }

    window
}



#[cfg(feature = "libadwaita")]
pub fn window(
    app: &gtk::Application,
    title: String,
    settings: Option<&gdk4::gio::Settings>,
    root: &gtk::Widget,
) -> libadwaita::ApplicationWindow {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title(&title)
        .content(root)
        .build();

    if let Some(settings) = settings {
        let w = 0.max(settings.get("window-width"));
        let h = 0.max(settings.get("window-height"));

        window.set_default_width(w);
        window.set_default_height(h);

        window.connect_close_request(glib::clone!(@strong settings => move |win| {
            let width = win.width();
            let height = win.height();
            settings.set_int("window-width", width - 122).unwrap();
            settings.set_int("window-height", height - 122).unwrap();
            gtk::Inhibit(false)
        }));
    }

    window
}
