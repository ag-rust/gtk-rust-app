# Rust GTK App Framework

This libaray aims to provide a frame for adaptive GTK4 and libadwaita apps written in rust.

Writing flatpak apps requires a lot of files whose content can be derived. Furthermore most adaptive GTK Apps will follow similar patterns. This project should get you started.

## Getting started

Define app metadata and the dependency to `gtk_app_framework` in your Cargo.toml:

```toml
# Cargo.toml

[package]
...

[app]
id = "org.example.FlatpakExample"
metadata-license = "CC0-1.0"
summary = "A flatpak app"
description = "This is an example flatpak application."
categories = [ "GTK", "Development" ]

[settings]
window-width = 600
window-height = 600

[actions]
quit = { accelerators = [ "<primary>W" ] }

[dependencies]
gtk-app-framework = { git = "https://gitlab.com/loers/gtk-rust-app.git" }
...
[build-dependencies]
gtk-app-framework = { git = "https://gitlab.com/loers/gtk-rust-app.git", features = [ "build" ] }

```

Create the file main.rs:

```rust
// src/main.rs
#[macro_use]
extern crate log;

use gtk::prelude::*;

mod views;

fn main() {
    env_logger::init();

    gtk_app_framework::builder()
        .view(
            {
                let home = gtk::Box::new(gtk::Orientation::Vertical, 0);
                home.append(&gtk::Label::new(Some(&gettextrs::gettext("Hello world!"))));
                home
            },
            Some("Home"),
            "home",
            Some("go-home-symbolic"),
        )
        .view(
            {
                let profile = gtk::Box::new(gtk::Orientation::Vertical, 0);
                profile.append(&gtk::Label::new(Some(&gettextrs::gettext("Hello you!"))));
                profile
            },
            Some("Profile"),
            "profile",
            Some("system-switch-user-symbolic"),
        )
        .build();
}

```

Call the build script:

```rust
// build.rs

pub fn main() {
    gtk_app_framework::build();
}
```

That's it. You will see an app like this:


![screenshot1.png](screenshot1.png)

The app has adaptive behaviour per default.

![screenshot2.png](screenshot2.png)

## Run with different language

```sh
LANGUAGE="de_DE:de" LANG="de_DE.utf8" TEXT_DOMAIN="target" cargo run
```

## Build

```sh
# build your binary release
cargo build --release
# create a flatpak
make flat
```

## Requirements

To build a flatpak you need the to install gnome-nightly remote

```sh
flatpak --user remote-add gnome-nightly gnome-nightly.flatpakrepo
```

