use gdk4::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::prelude::WidgetExt;

pub mod macros;
pub mod components;
mod window;

pub use window::*;

pub fn run(
    app_id: &str,
    window: impl Fn(&gtk::Application) -> gtk::ApplicationWindow + 'static,
    connect_startup: impl Fn(&gtk::Application) + 'static,
) {
    let app = gtk::Application::builder().application_id(app_id).build();
    let resource_base_path = format!("/{}/", app_id.replace(".", "/"));
    app.set_resource_base_path(Some(&resource_base_path));
    app.connect_activate(move |app| {
        let window = window(app);
        window.show();
    });
    app.connect_startup(move |app| {
        connect_startup(app);
    });
    app.run();
}

pub fn load_styles(_app: &gtk::Application, styles: &str) {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(styles.as_bytes());
    gtk::StyleContext::add_provider_for_display(
        &gdk4::Display::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
