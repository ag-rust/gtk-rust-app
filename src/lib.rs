// SPDX-License-Identifier: GPL-3.0-or-later

#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[cfg(feature = "ui")]
pub mod builder;

#[cfg(feature = "ui")]
mod ui;

#[cfg(feature = "ui")]
pub use gtk_rust_app_derive::*;
#[cfg(feature = "ui")]
pub use ui::widgets;
#[cfg(feature = "ui")]
pub use ui::window::window;

pub use once_cell;
pub use serde_json;

#[cfg(feature = "store")]
pub use gstore;

pub use builder::builder as app;

#[cfg(feature = "ui")]
pub fn load_resources(resource_bytes: &[u8]) {
    let res = gdk4::gio::Resource::from_data(&resource_bytes.into())
        .expect("Could not load gresource file");
    gdk4::gio::resources_register(&res);
}

#[cfg(feature = "ui")]
pub fn init_gettext(domain: &str) {
    let textdomain = match std::env::var("TEXT_DOMAIN") {
        Ok(path) => gettextrs::TextDomain::new(domain)
            .skip_system_data_paths()
            .push(&path)
            .init(),
        Err(_) => gettextrs::TextDomain::new(domain).init(),
    };
    match &textdomain {
        Ok(locale) => match locale {
            Some(_locale) => {
                // nothing to do
            },
            None => eprintln!("Warning: No locale was set! Probably /usr/share/locale/*/LC_MESSAGES does not contain a .mo file.")
        },
        Err(e) => match e {
            gettextrs::TextDomainError::InvalidLocale(locale) => eprintln!("Warning: Invalid locale {:?}", locale),
            gettextrs::TextDomainError::TranslationNotFound(locale) => {
                warn!("Could not find messages for locale {:?}, text domain: {:?}, TEXT_DOMAIN: {:?}", 
                    locale,
                    textdomain,
                    std::env::var("TEXT_DOMAIN"))
            },
            e => {
                error!("Text domain error: {}", e);
            }
        }
    };
}
