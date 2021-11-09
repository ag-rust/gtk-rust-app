#[macro_use]
extern crate log;

#[macro_use]
extern crate gstore;

#[macro_use]
extern crate gtk_rust_app;

use gdk4::{
    gio::SimpleAction,
    prelude::{ActionMapExt, ApplicationExt},
};
use gettextrs::gettext;
use glib::Cast;

mod components;
mod store;
mod views;

use store::{Action, State, UIState};

fn main() {
    env_logger::init();

    info!("Starting");

    // Handle actions and modify the global state
    store::init_store(|action, mut state| {
        let next_state = match action {
            Action::CreateEvent => state,
            Action::EventCreated(event) => {
                let default_calendar = state.calendars.get_mut(0).unwrap();
                default_calendar.events.push(event.clone());
                state
            }

            Action::Select(selection) => State {
                selection: selection.clone(),
                ..state
            },

            Action::ChangeMobile(mobile) => State {
                ui: UIState {
                    mobile: *mobile,
                    ..state.ui
                },
                ..state
            },
            Action::ChangedView(current_view) => {
                let mut navigation_history = state.ui.navigation_history.clone();
                let last = navigation_history.len() - 1;
                navigation_history[last] = current_view.clone();
                
                State {
                    ui: UIState {
                        navigation_history,
                        ..state.ui
                    },
                    ..state
                }
            },
            Action::Navigate(navigation) => {
                let mut navigation_history = state.ui.navigation_history.clone();

                if navigation == "back" {
                    if navigation_history.len() > 1 {
                        navigation_history.remove(navigation_history.len() - 1);
                    }
                } else {
                    navigation_history.push(navigation.clone());
                }

                State {
                    ui: UIState {
                        navigation_history,
                        ..state.ui
                    },
                    ..state
                }
            }
        };
        println!("{:?}", next_state);
        next_state
    });

    // build your app with a builder pattern
    gtk_rust_app::builder()
        .title(gettext("Calendar"))
        .with_settings()
        //
        // setting a stylesheet
        .styles(include_str!("../assets/styles.css"))
        //
        // define views
        .view(
            vec![
                views::open_create_event_view_header_button(),
                views::prev_month_header_button(),
                views::next_month_header_button(),
            ],
            vec![],
            views::month(),
            "month",
            Some((gettext("Month"), "month-symbolic".into())),
        )
        .view(
            Vec::new(),
            Vec::new(),
            views::week(),
            "week",
            Some((gettext("Week"), "week-symbolic".into())),
        )
        //
        // define views without implicit navigation entries
        .view(
            vec![views::create_event_save_header_button()],
            Vec::new(),
            views::create_event(),
            "create-event",
            // adding a view without title and icon will not show the view in the navigation bars.
            None,
        )
        //
        // Register some connectors to the content leaflet.
        .connect_leaflet(|leaflet| {
            leaflet.connect_folded_notify(|leaflet| {
                dispatch!(leaflet, Action::ChangeMobile(leaflet.is_folded()))
            });
        })
        //
        // Register some connectors to the view stack
        .connect_view_stack(|view_stack| {
            view_stack.connect_visible_child_name_notify(|view_stack| {
                dispatch!(
                    view_stack,
                    Action::ChangedView(view_stack.visible_child_name().unwrap().into())
                )
            });
            store::store().select(glib::clone!(@weak view_stack => move |action, state| {
                if let Some(Action::Navigate(_)) = action {
                    view_stack.set_visible_child_name(&state.ui.navigation_history.last().unwrap());
                }
            }));
        })
        //
        // connect the store to the gtk application for gtk action delegation
        .store(store::store())
        //
        // build the app and handle non-domain actions like quit
        .build(
            |app| {
                if let Some(action) = app.lookup_action("quit") {
                    let simple_action: SimpleAction = action.downcast().unwrap();
                    simple_action.connect_activate(glib::clone!(@weak app => move |_, _| {
                        app.quit();
                    }));
                }
            },
            |_| {},
        );
}
