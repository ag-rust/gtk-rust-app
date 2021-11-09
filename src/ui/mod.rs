use gdk4::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::prelude::WidgetExt;
use libadwaita as adw;

pub mod components;
mod window;
pub use window::*;

pub struct View {
    widget: gtk::Box,
    title: String,
    name: Option<String>,
    icon_name: Option<String>,
}

pub fn run(
    app_id: &str,
    window: impl Fn(&adw::Application) -> adw::ApplicationWindow,
    connect_startup: impl Fn(&adw::Application),
) {
    gtk::init().expect("Couldn't initialize GTK");
    adw::init();

    let app = adw::Application::builder().application_id(app_id).build();

    app.connect_activate(move |app| {
        let window = window(app);
        window.show();
    });

    app.connect_startup(move |app| {
        connect_startup(app);
    });

    app.run();
}

pub fn load_styles(app: &adw::Application, styles: &str) {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(styles);
    gtk::StyleContext::add_provider_for_display(
        &gdk4::Display::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
