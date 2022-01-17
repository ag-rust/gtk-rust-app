// SPDX-License-Identifier: GPL-3.0-or-later

#[allow(unused_imports)]
#[macro_use]
extern crate log;

mod descriptor;
pub use descriptor::*;

#[cfg(feature = "ui")]
pub mod builder;

#[cfg(feature = "ui")]
mod ui;
#[cfg(feature = "ui")]
pub use gtk_rust_app_derive::*;
#[cfg(feature = "ui")]
pub use ui::components;
#[cfg(feature = "ui")]
pub use ui::window::window;

// #[cfg(feature = "build")]
pub mod build;

pub use once_cell;
pub use regex;
pub use serde_json;

#[cfg(feature = "store")]
pub use gstore;

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

#[cfg(feature = "build")]
pub fn build(output_dir: Option<&std::path::Path>) {
    use crate::build::{
        build_actions, build_flatpak, build_gettext, build_gresources, build_gschema_settings,
        build_makefile,
    };

    let project_descriptor = parse_project_descriptor(std::path::Path::new("Cargo.toml"))
        .expect("Could not read Cargo.toml");

    let target = output_dir.unwrap_or(&std::path::Path::new("target/gra-gen"));
    std::fs::create_dir_all(target).expect("Could not create out dir.");

    build_actions(&project_descriptor, &target);
    build_gschema_settings(&project_descriptor, &target);
    build_flatpak(&project_descriptor, &target);
    build_gresources(&project_descriptor, &target);
    build_makefile(&project_descriptor, &target);
    build_gettext(&project_descriptor, &target);
}
