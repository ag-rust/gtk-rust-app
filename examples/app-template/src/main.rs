#[macro_use]
extern crate log;

mod ui;

use gdk4::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::prelude::WidgetExt;
use libadwaita as adw;

const APP_ID: &str = "org.project.App";

fn main() {
    env_logger::init();
    gtk::init().expect("Couldn't initialize GTK");
    adw::init();
    init_gettext();

    let app = libadwaita::Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        let window = ui::window::window(&app);
        window.show();
    });

    app.connect_startup(|_app| {
        let provider = gtk::CssProvider::new();
        provider.load_from_data(include_bytes!("ui/styles.css"));
        gtk::StyleContext::add_provider_for_display(
            &gdk4::Display::default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    app.run();
}

fn init_gettext() {
    let domain = APP_ID;
    let textdomain = match std::env::var("TEXT_DOMAIN") {
        Ok(path) => gettextrs::TextDomain::new(domain)
            .skip_system_data_paths()
            .push(&path)
            .init(),
        Err(_) => gettextrs::TextDomain::new(domain).init(),
    };
    match textdomain {
        Ok(locale) => match locale {
            Some(_locale) => {
                // nothing to do
            },
            None => eprintln!("Warning: No locale was set! Probably /usr/share/locale/*/LC_MESSAGES does not contain a .mo file.")
        },
        Err(e) => match e {
            gettextrs::TextDomainError::InvalidLocale(locale) => eprintln!("Warning: Invalid locale {:?}", locale),
            gettextrs::TextDomainError::TranslationNotFound(locale) => match locale.as_str() {
                "en" => {
                    // use default language
                },
                _ => error!("Warning: Could not find messages for locale {:?}", locale)
            },
            e => {
                error!("{:?}", e);
            }
        }
    };
}
