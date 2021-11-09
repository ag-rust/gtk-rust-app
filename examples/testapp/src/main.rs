#[macro_use]
extern crate log;

use gdk4::prelude::{ApplicationExt, ApplicationExtManual};
use gettextrs::gettext;
use gtk::prelude::WidgetExt;
use libadwaita as adw;

const APP_ID: &str = "org.project.Testapp";

fn main() {
    gtk::init().expect("Couldn't initialize GTK");
    adw::init();
    
    gtk_app_framework::init_gettext(APP_ID);

    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app| {
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Example App")
            .build();
        // window.set_child(Some(&gtk::Label::new(Some(&gettext("Hello World")))))
    });

    app.run();
}
