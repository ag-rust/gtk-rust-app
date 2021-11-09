#[macro_use]
extern crate log;

use gettextrs::gettext;
use gtk::prelude::*;

fn main() {
    env_logger::init();

    debug!("{}", gettext("Hello flatpak"));
    
    gtk_rust_app::builder()
        .default_window(false)
        .build(
            |app| {
                if let Some(action) = app.lookup_action("quit") {
                    let quit_action: gdk4::gio::SimpleAction = action.downcast().unwrap();
                    quit_action.connect_activate(glib::clone!(@weak app => move |_, _| {
                        app.quit();
                    }));
                }
            },
            move |app| {
                let window = gtk::ApplicationWindow::builder()
                    .default_width(400)
                    .default_height(400)
                    .application(app)
                    .build();
                let label = gtk::Label::new(Some(&gettext("Hello flatpak")));
                window.set_child(Some(&label));
                window.show();
            },
        );
}
