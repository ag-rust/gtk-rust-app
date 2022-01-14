// SPDX-License-Identifier: GPL-3.0-or-later

use gtk::prelude::*;
use libadwaita as adw;

pub fn window(
    app: &gtk::Application,
    title: String,
    settings: Option<&gdk4::gio::Settings>,
    root: &gtk::Widget,
) -> adw::ApplicationWindow {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title(&title)
        .content(root)
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

    return window;
}
