// SPDX-License-Identifier: GPL-3.0-or-later

use gdk4::gio::SimpleAction;
use gdk4::prelude::{ApplicationExt, ApplicationExtManual};
use glib::VariantTy;
use gra::{parse_project_descriptor_bytes, ProjectDescriptor};
use gtk::prelude::GtkApplicationExt;
use gtk::prelude::*;
#[cfg(feature = "ui")]
use libadwaita as adw;

use crate::{init_gettext, load_resources};

pub fn load_styles(_app: &gtk::Application, styles: &str) {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(styles.as_bytes());
    gtk::StyleContext::add_provider_for_display(
        &gdk4::Display::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub struct AppBuilder {
    project_descriptor: ProjectDescriptor,

    app: gtk::Application,
    settings: Option<gdk4::gio::Settings>,

    styles: Option<&'static str>,

    #[cfg(feature = "store")]
    delegate_store: Option<glib::Sender<(SimpleAction, Option<glib::Variant>)>>,
}

impl AppBuilder {
    pub fn build(
        self,
        startup: impl Fn(&gtk::Application, &ProjectDescriptor, Option<&gdk4::gio::Settings>) + 'static,
        activate: impl Fn(&gtk::Application, &ProjectDescriptor, Option<&gdk4::gio::Settings>) + 'static,
    ) {
        let project_descriptor = self.project_descriptor;
        let settings = self.settings;
        let app = self.app;
        let styles = self.styles;

        #[cfg(feature = "store")]
        let delegate_store = self.delegate_store;

        let pd = project_descriptor.clone();
        let s = settings.clone();
        app.connect_activate(move |app| {
            activate(app, &pd, s.as_ref());
        });

        app.connect_startup(move |app| {
            if let Some(styles) = styles {
                load_styles(app, styles);
            }

            let actions = project_descriptor.actions.as_ref().unwrap();

            for (action_name, desc) in actions {
                let action = match &desc.type_ {
                    Some(action_type) => {
                        let t = VariantTy::new(action_type).unwrap_or_else(|e| {
                            panic!(
                                "Wrong type for action '{}', {:?}: {}",
                                action_name, action_type, e
                            )
                        });
                        SimpleAction::new(action_name, Some(t))
                    }
                    None => SimpleAction::new(action_name, None),
                };

                #[cfg(feature = "store")]
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

            startup(app, &project_descriptor, settings.as_ref());
        });

        app.run();
    }

    #[cfg(feature = "store")]
    pub fn store<S: std::fmt::Debug + Clone + Default + PartialEq + Eq>(
        mut self,
        store: &'static mut gstore::Store<S>,
    ) -> Self {
        self.delegate_store = Some(store.delegate());
        self
    }

    pub fn styles(mut self, styles: &'static str) -> Self {
        self.styles = Some(styles);
        self
    }
}

pub fn builder(cargo_toml: &[u8], app_toml: &[u8], resources: &[u8]) -> AppBuilder {
    if let Err(e) = gtk::init() {
        error!("Couldn't initialize GTK: {:?}", e);
    }
    #[cfg(feature = "ui")]
    adw::init();

    let project_descriptor = parse_project_descriptor_bytes(cargo_toml, app_toml);
    if project_descriptor.is_err() {
        panic!(
            "Could not parse Cargo.toml: {}",
            project_descriptor.unwrap_err()
        );
    }
    let project_descriptor = project_descriptor.unwrap();

    let app_desc = &project_descriptor.app;

    let app = gtk::Application::builder()
        .application_id(&app_desc.id)
        .build();
    let resource_base_path = format!("/{}/", app_desc.id.replace('.', "/"));
    app.set_resource_base_path(Some(&resource_base_path));

    load_resources(resources);

    init_gettext(&project_descriptor.package.name);

    let settings = Some(gdk4::gio::Settings::new(&project_descriptor.app.id));

    AppBuilder {
        project_descriptor,
        app,
        settings,
        styles: None,
        #[cfg(feature = "store")]
        delegate_store: Default::default(),
    }
}
