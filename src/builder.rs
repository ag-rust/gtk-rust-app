// SPDX-License-Identifier: GPL-3.0-or-later

use gdk4::gio::SimpleAction;
use gdk4::prelude::{ApplicationExt, ApplicationExtManual};
use glib::VariantTy;
use gra::{parse_project_descriptor_bytes, ProjectDescriptor};
use gtk::builders::ApplicationBuilder;
use gtk::prelude::GtkApplicationExt;
use gtk::prelude::*;

use crate::{init_gettext, load_resources};

/// Load the given css styles for your app.
pub fn load_styles(_app: &gtk::Application, styles: &str) {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(styles.as_bytes());
    gtk::StyleContext::add_provider_for_display(
        &gdk4::Display::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// The root application builder. The AppBuilder allows to setup everything based on toml files and metadata.
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
        add_gstore_debug_action(&self.app, store);
        self.delegate_store = Some(store.delegate());
        self
    }

    pub fn styles(mut self, styles: &'static str) -> Self {
        self.styles = Some(styles);
        self
    }
}

#[cfg(feature = "store")]
static DEBUG_SENDER: once_cell::sync::OnceCell<glib::Sender<(gstore::Action, String)>> =
    once_cell::sync::OnceCell::new();

#[cfg(not(debug_assertions))]
fn add_gstore_debug_action<S: std::fmt::Debug + Clone + Default + PartialEq + Eq>(
    application: &gtk::Application,
    store: &mut gstore::Store<S>,
) {
}

#[cfg(debug_assertions)]
#[cfg(feature = "store")]
fn add_gstore_debug_action<S: std::fmt::Debug + Clone + Default + PartialEq + Eq>(
    application: &gtk::Application,
    store: &mut gstore::Store<S>,
) {
    use glib::PRIORITY_DEFAULT;
    use gstore::Middleware;

    struct GstoreDebuggingMiddleware;
    let m = GstoreDebuggingMiddleware;

    let (send, receiver) = glib::MainContext::channel(PRIORITY_DEFAULT);
    if DEBUG_SENDER.set(send).is_err() {
        error!("Failed to set up gstore debugging UI: Did you try to initialize gstore twice?");
    }

    impl<S: std::fmt::Debug + Clone + Default + PartialEq + Eq + 'static> Middleware<S>
        for GstoreDebuggingMiddleware
    {
        fn post_reduce(&self, a: &gstore::Action, s: &S) {
            if let Some(sender) = DEBUG_SENDER.get() {
                if let Err(e) = sender.send((a.clone(), format!("{:#?}", s))) {
                    println!("Failed to delegate action to gstore debugging: {}", e)
                }
            } else {
                println!("Failed to send action to gstore debugging UI.")
            }
        }
    }

    let name = &"gstore-debug";
    let action = gdk4::gio::SimpleAction::new(name, None);
    let w = create_debug_window(receiver);
    store.append_middleware(Box::new(m));
    action.connect_activate(glib::clone!(@weak application, @weak w => move |_, _| {
        w.show();
    }));
    application.set_accels_for_action(&format!("app.{}", name), &["<primary><alt>G"]);
    application.add_action(&action);
}

#[cfg(debug_assertions)]
fn create_debug_window(recv: glib::Receiver<(gstore::Action, String)>) -> libadwaita::Window {
    let d = crate::ui::debugging::GstoreDebug::new(Some(recv));
    libadwaita::Window::builder()
        .default_height(600)
        .default_width(500)
        .hide_on_close(true)
        .content(&d)
        .build()
}

///
/// Root setup function for your GTK application.
///
/// # Arguments
///
/// - `cargo_toml`: The source of your Cargo.toml (required to get the app binary name).
/// - `app_toml`: The source of your App.toml (to get all kinds of meta data, settings, actions etc.).
/// - `resources`: Compiled gtk resources (like icons).
/// - `app`: Optionally you can provide your own gtk::ApplicationBuilder instance for a customized app setup.
///
/// # Example
///
/// Also [read this](https://gitlab.com/floers/gtk-rust-app/-/blob/main/examples/simple/src/main.rs) for a complete example.
///
/// ```no_run
/// use gettextrs::gettext;
/// use gtk::prelude::*;
/// use gtk_rust_app::*;
///
/// gtk_rust_app::app(
///     &[], //include_bytes!("../Cargo.toml"),
///     &[], //include_bytes!("../App.toml"),
///     &[], //include_bytes!("../target/gra-gen/compiled.gresource"),
///     None,
/// )
/// // include your style sheets here
/// //.styles(include_str!("styles.css"))
/// .build(
/// |application, _project_descriptor, settings| {
///     // setup custom types
///     // my::Widget::static_type();
///
///     // The pages will be placed in this predefined adaptive layout.
///     let leaflet_layout = gtk_rust_app::widgets::LeafletLayout::builder(settings)
///         // .add_page(pages::my_page())
///         .build();
///
///     // LeafletLayout contains a toast overlay
///     leaflet_layout.show_message("Hello world");
///
///     // and we use the leaflet layout as root content in the apps window.
///     let window = gtk_rust_app::window(
///         application,
///         gettext("Example"),
///         settings,
///         leaflet_layout.upcast_ref(),
///     );
///     window.show();
/// },
/// |app, _project_descriptor, _settings| {
///     if let Some(action) = app.lookup_action("quit") {
///         let simple_action: gdk4::gio::SimpleAction = action.downcast().unwrap();
///         simple_action.connect_activate(glib::clone!(@weak app => move |_, _| {
///             app.quit();
///         }));
///     }
/// },
/// );
/// ```
pub fn builder(
    cargo_toml: &[u8],
    app_toml: &[u8],
    resources: &[u8],
    app: Option<ApplicationBuilder>,
) -> AppBuilder {
    if let Err(e) = gtk::init() {
        error!("Couldn't initialize GTK: {:?}", e);
    }
    #[cfg(feature = "libadwaita")]
    libadwaita::init();

    let project_descriptor = parse_project_descriptor_bytes(cargo_toml, app_toml);
    if project_descriptor.is_err() {
        panic!(
            "Could not parse Cargo.toml: {}",
            project_descriptor.unwrap_err()
        );
    }
    let project_descriptor = project_descriptor.unwrap();

    let app_desc = &project_descriptor.app;

    let app = app
        .unwrap_or_else(gtk::Application::builder)
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
