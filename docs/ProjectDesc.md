# Project template description

This document explains the default project structure and where additional code is generated.
# Code structure

Using `gtk-rust-app` a initial project setup should look like this:

```txt
Cargo.toml
- assets/
  - icons/
  
    - ...                    // any icons you want to use in your app
  
  - icon.64.png              // These three icons are your app icon 
                             // used for the flatpak app
  - icon.128.png

  - icon.svg

- po/

  - LINGUAS                  // lines of locales to translate to

  - <locale>.po              // the gettext translation file (Generated 
                             // by usages of gettext("...") in the code)

- src/

  - main.rs                  // The entry point as usual

  - store.rs                 // Global state and domain structs definition

  - pages/                   // module for all pages

    - home.rs

    - ...

  - components/              // Module for reusable components

    - back_header_button.rs

    - ...

Cargo.toml                   // The common rust project metadata as well
                             // as flatpak metadata and GTK settings and
                             // actions. See (1)

```

- (1): [Manifest](docs/Manifest.md)

# Generated sources and resources

GTK apps require several files which are derived from the extended `Cargo.toml`. These files can be generated using `cargo gra setup`. The following file tree will be generated:

```
target/gra-gen
 - assets                                   // assets xml file

 - data                                     // flatpak metadata .yml, .desktop, 
                                            // as well as app icon svg and png 
                                            // files - eveything installed in flatpak

 - locale                                   // gettext translation context file 
                                            // structure to use for in-dev testing

 - po                                       // Generated gettext translation .pot files.

 - actions.rs                               // Generated constants for your global actions

 - compiled.gresources                      // compiled binary of your resources

 - Makefile                                 // Used to install the app inside the 
                                            // flapak container during a flatpak build

 - org.example.SimpleExample.gschema.xml    // Schema for your app's gsettings
```
