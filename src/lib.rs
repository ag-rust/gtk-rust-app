// SPDX-License-Identifier: GPL-3.0-or-later

#[allow(unused_imports)]
#[macro_use]
extern crate log;

mod descriptor;
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub use descriptor::*;

#[cfg(feature = "ui")]
pub mod builder;

#[cfg(feature = "ui")]
mod ui;
use fs_extra::dir::CopyOptions;
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

    let project_descriptor = parse_project_descriptor(std::path::Path::new("Cargo.toml"));

    if project_descriptor.is_err() {
        eprintln!(
            "[gra] Could not parse Cargo.toml: {:?}",
            project_descriptor.unwrap_err()
        );
        return;
    }
    let project_descriptor = project_descriptor.unwrap();

    let target = output_dir.unwrap_or(&std::path::Path::new("target/gra-gen"));
    std::fs::create_dir_all(target).expect("Could not create out dir.");

    build_actions(&project_descriptor, &target);
    build_gschema_settings(&project_descriptor, &target);
    build_flatpak(&project_descriptor, &target);
    build_gresources(&project_descriptor, &target);
    build_makefile(&project_descriptor, &target);
    build_gettext(&project_descriptor, &target);
}

/// Prepare the flatpak-temp directory which may be used to build a flatpak app.
/// Returns a PathBuf to that directory.
pub fn prepare_flatpak_temp(project_dir: &PathBuf) -> Result<PathBuf, String> {
    println!("[gra] Prepare flatpak build...");

    let flatpak_temp = project_dir.join("target/flatpak-temp");

    // setup flatpak-temp dir
    // let flatpak_temp = target_dir.join("flatpak-temp");
    if flatpak_temp.exists() {
        remove_dir_all(&flatpak_temp).map_err(|e| e.to_string())?;
    }

    println!("[gra] mkdir target/flatpak-temp");
    create_dir_all(&flatpak_temp).map_err(|e| e.to_string())?;
    println!("[gra] mkdir target/flatpak-temp/target");
    create_dir_all(&flatpak_temp.join("target")).map_err(|e| e.to_string())?;
    println!("[gra] mkdir target/flatpak-temp/.cargo");
    create_dir_all(&flatpak_temp.join(".cargo")).map_err(|e| e.to_string())?;

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    println!("[gra] cp -r src target/flatpak-temp");
    fs_extra::dir::copy(project_dir.join("src"), &flatpak_temp, &options)
        .map_err(|e| e.to_string())?;
    println!("[gra] cp -r po target/flatpak-temp");
    fs_extra::dir::copy(project_dir.join("po"), &flatpak_temp, &options)
        .map_err(|e| e.to_string())?;
    println!("[gra] cp Cargo.toml target/flatpak-temp");
    std::fs::copy(
        project_dir.join("Cargo.toml"),
        &flatpak_temp.join("Cargo.toml"),
    )
    .map_err(|e| e.to_string())?;
    println!("[gra] cp -r target/gra-gen target/flatpak-temp/target");
    fs_extra::dir::copy(
        project_dir.join("target/gra-gen"),
        &flatpak_temp.join("target"),
        &options,
    )
    .map_err(|e| e.to_string())?;

    println!("[gra] Vendoring sources...");
    let c = Command::new("cargo")
        .current_dir(&flatpak_temp)
        .args(["vendor", "target/vendor"])
        .output()
        .map_err(|e| e.to_string())?;

    if let Ok(e) = String::from_utf8(c.stderr) {
        if !e.trim().is_empty() {
            println!("[gra] {}", e);
        }
    }
    let mut config =
        File::create(flatpak_temp.join(".cargo").join("config.toml")).map_err(|e| e.to_string())?;
    config.write_all(&c.stdout).map_err(|e| e.to_string())?;

    Ok(flatpak_temp)
}
