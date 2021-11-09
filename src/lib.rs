#[macro_use]
extern crate log;

mod descriptor;

pub mod build;
pub mod ui;

pub use descriptor::*;

pub struct AppBuilder {
    views: Vec<gtk::Box>,
    styles: Option<&'static str>,
}

impl AppBuilder {
    pub fn build(&self) {
        let project_descriptor =
            configs::parse_project_descriptor().expect("Could not parse projects Cargo.toml");
        let app_id = &project_descriptor.flatpak.package_name;

        init_gettext(app_id);

        let styles = self.styles;
        let settings = gdk4::gio::Settings::new(app_id);
        let views = self.views;

        ui::run(
            app_id,
            move |app| ui::window(project_descriptor, app, views, settings),
            move |app| {
                if let Some(styles) = styles {
                    ui::load_styles(app, styles);
                }
            },
        );
    }

    pub fn load_styles(&mut self, styles: &'static str) {
        self.styles = styles;
    }
}

pub fn builder() -> AppBuilder {
    AppBuilder {
        styles: None,
        views: Vec::new(),
    }
}

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

pub fn build() {
    println!("cargo:rerun-if-changed=src");
    build_gettext();
    build_flatpak();
}
