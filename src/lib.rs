#[macro_use]
extern crate log;

use gdk4::prelude::{ApplicationExt, ApplicationExtManual};
use gdk4::{gio::SimpleAction, prelude::ActionMapExt};
use glib::VariantTy;
use gtk::prelude::{GtkApplicationExt, WidgetExt};
use libadwaita as adw;
use std::path::Path;

mod descriptor;
pub use descriptor::*;

pub mod ui;
use ui::View;
#[cfg(feature = "build")]
pub mod build;
#[cfg(feature = "build")]
use crate::build::{
    build_flatpak, build_gettext, build_gresources, build_gschema_settings, build_makefile,
};

pub use once_cell;
pub use serde_json;

pub struct AppBuilder {
    project_descriptor: ProjectDescriptor,
    title: Option<String>,

    app: gtk::Application,
    settings: Option<gdk4::gio::Settings>,

    default_window: bool,

    header_sidebar: Option<Option<adw::HeaderBar>>,
    header_main: Option<Option<adw::HeaderBar>>,

    sidebar: Option<Box<dyn Fn(&adw::ViewStack) -> gtk::Widget + 'static>>,
    view_switcher_bar: Option<Box<dyn Fn(&adw::ViewStack) -> adw::ViewSwitcherBar + 'static>>,

    views: Vec<View>,
    styles: Option<&'static str>,

    connect_leaflet: Option<Box<dyn Fn(&adw::Leaflet) + 'static>>,
    connect_view_stack: Option<Box<dyn Fn(&adw::ViewStack) + 'static>>,

    delegate_store: Option<glib::Sender<(SimpleAction, Option<glib::Variant>)>>,
}

impl AppBuilder {
    pub fn build(
        self,
        startup: impl Fn(&gtk::Application) + 'static,
        activate: impl Fn(&gtk::Application) + 'static,
    ) {
        let project_descriptor = self.project_descriptor;
        let title = self.title;
        let package_name = project_descriptor.package.name.clone();
        let actions = project_descriptor.actions;

        let settings = self.settings;

        let app = self.app;
        let default_window = self.default_window.clone();

        let default_header_sidebar = Some(ui::components::header_sidebar(&settings));
        let default_header_main = Some(ui::components::header_main(&settings));
        let header_sidebar = self.header_sidebar.unwrap_or(default_header_sidebar);
        let header_main = self.header_main.unwrap_or(default_header_main);

        let views = self.views.clone();
        let sidebar = self.sidebar;
        let view_switcher_bar = self.view_switcher_bar;

        let styles = self.styles;

        #[cfg(feature = "store")]
        let delegate_store = self.delegate_store;

        let connect_leaflet = self.connect_leaflet;
        let connect_view_stack = self.connect_view_stack;

        app.connect_activate(move |app| {
            if default_window {
                let window = ui::window(
                    &package_name,
                    &title,
                    &app,
                    &views,
                    &header_sidebar,
                    &header_main,
                    &sidebar,
                    &view_switcher_bar,
                    &settings,
                    &connect_leaflet,
                    &connect_view_stack,
                );
                window.show();
                activate(app);
            } else {
                activate(app);
            }
        });

        app.connect_startup(move |app| {
            for (action_name, desc) in &actions {
                let action = match &desc.action_type {
                    Some(action_type) => unsafe {
                        SimpleAction::new(
                            &action_name,
                            Some(&VariantTy::from_str_unchecked(&action_type)),
                        )
                    },
                    None => SimpleAction::new(&action_name, None),
                };

                let delegate = delegate_store.clone();
                #[cfg(feature = "store")]
                action.connect_activate(move |action, argument| {
                    if let Some(delegate) = &delegate {
                        delegate
                            .send((action.clone(), argument.cloned()))
                            .expect("Could not delegate action to store!");
                    }
                });

                if let Some(accelerators) = &desc.accelerators {
                    let mut accels = Vec::new();
                    for accel in accelerators {
                        accels.push(accel.as_str());
                    }
                    app.set_accels_for_action(&format!("app.{}", action_name), &accels);
                }
                app.add_action(&action);
            }
            if let Some(styles) = styles {
                ui::load_styles(app, styles);
            }
            startup(app);
        });
        app.run();
    }

    pub fn with_settings(mut self) -> Self {
        self.settings = Some(gdk4::gio::Settings::new(&self.project_descriptor.app.id));
        self
    }

    pub fn default_window(mut self, default_window: bool) -> Self {
        self.default_window = default_window;
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    #[cfg(feature = "store")]
    pub fn store<
        A: gstore::Action + std::fmt::Debug + Clone,
        S: std::fmt::Debug + Clone + Default,
    >(
        mut self,
        store: &'static gstore::Store<A, S>,
    ) -> Self
    where
        A: serde::de::DeserializeOwned,
    {
        self.delegate_store = Some(store.delegate());
        self
    }

    pub fn view_switcher_bar(
        mut self,
        view_switcher_bar: impl Fn(&adw::ViewStack) -> adw::ViewSwitcherBar + 'static,
    ) -> Self {
        self.view_switcher_bar = Some(Box::new(view_switcher_bar));
        self
    }

    pub fn sidebar(mut self, sidebar: impl Fn(&adw::ViewStack) -> gtk::Widget + 'static) -> Self {
        self.sidebar = Some(Box::new(sidebar));
        self
    }

    pub fn view(
        mut self,
        start_header_widgets: Vec<gtk::Widget>,
        end_header_widgets: Vec<gtk::Widget>,
        view: gtk::Widget,
        name: &'static str,
        title_and_icon: Option<(String, String)>,
    ) -> Self {
        self.views.push(View {
            start_header_widgets,
            end_header_widgets,
            widget: view,
            name,
            title_and_icon,
        });
        self
    }

    pub fn settings(mut self, settings: Option<gdk4::gio::Settings>) -> Self {
        self.settings = settings;
        self
    }

    pub fn styles(mut self, styles: &'static str) -> Self {
        self.styles = Some(styles);
        self
    }

    // connectors
    pub fn connect_leaflet(mut self, connector: impl Fn(&adw::Leaflet) + 'static) -> Self {
        self.connect_leaflet = Some(Box::new(connector));
        self
    }
    pub fn connect_view_stack(mut self, connector: impl Fn(&adw::ViewStack) + 'static) -> Self {
        self.connect_view_stack = Some(Box::new(connector));
        self
    }
}

pub fn builder() -> AppBuilder {
    if let Err(e) = gtk::init() {
        error!("Couldn't initialize GTK: {:?}", e);
    }
    adw::init();

    let project_descriptor = parse_project_descriptor(Path::new("Cargo.toml"))
        .expect("Could not parse projects Cargo.toml");

    let app = gtk::Application::builder()
        .application_id(&project_descriptor.app.id)
        .build();
    let resource_base_path = format!("/{}/", project_descriptor.app.id.replace(".", "/"));
    app.set_resource_base_path(Some(&resource_base_path));
    load_resources();

    init_gettext(&project_descriptor.package.name);

    AppBuilder {
        project_descriptor,
        title: None,
        app,
        settings: None,
        default_window: true,
        header_sidebar: None,
        header_main: None,
        sidebar: Some(Box::new(|view_stack| ui::components::sidebar(view_stack))),
        view_switcher_bar: Some(Box::new(|view_stack| {
            ui::components::view_switcher_bar(view_stack)
        })),
        views: Default::default(),

        styles: None,

        connect_leaflet: None,
        connect_view_stack: None,

        #[cfg(feature = "store")]
        delegate_store: Default::default(),
    }
}

pub fn load_resources() {
    let res = gdk4::gio::Resource::load("out/assets/compiled.gresource")
        .expect("Could not load gresource file");
    gdk4::gio::resources_register(&res);
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

#[cfg(feature = "build")]
pub fn build() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=assets");
    println!("cargo:rerun-if-changed=po");

    let project_descriptor =
        parse_project_descriptor(Path::new("Cargo.toml")).expect("Could not read Cargo.toml");

    let target = Path::new("out");

    build_gettext(&project_descriptor, &target);
    build_gschema_settings(&project_descriptor, &target);
    build_flatpak(&project_descriptor, &target);
    build_gresources(&project_descriptor, &target);
    build_makefile(&project_descriptor, &target);
}
