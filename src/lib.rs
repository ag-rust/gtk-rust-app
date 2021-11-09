#[macro_use]
extern crate log;

mod flatpak;
mod i18n;

pub use i18n::{build_gettext, init_gettext};
pub use flatpak::build_flatpak;

pub fn build() {
    println!("cargo:rerun-if-changed=src");
    build_gettext();
    build_flatpak();
}