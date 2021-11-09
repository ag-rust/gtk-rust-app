use chrono::prelude::*;
use serde::{Deserialize, Serialize};

// Action
// ------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Action {
    CreateEvent,
    EventCreated(CalendarEvent),
    Select(Option<(DateTime<Local>, DateTime<Local>)>),

    ChangeMobile(bool),
    ChangedView(String),
    Navigate(String),
}

impl gstore::Action for Action {
    fn name(&self) -> &str {
        match self {
            Action::CreateEvent => "app.create-event",
            Action::EventCreated(_) => "app.event-created",
            Action::Select(_) => "app.select",

            Action::ChangeMobile(_) => "app.ui",
            Action::ChangedView(_) => "app.ui",
            Action::Navigate(_) => "app.ui",
        }
    }
}

// State
// ------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Calendar {
    pub events: Vec<CalendarEvent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CalendarEvent {
    pub name: String,
    pub location: String,
    pub start: DateTime<chrono::Local>,
    pub end: DateTime<chrono::Local>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UIState {
    pub mobile: bool,
    pub navigation_history: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct State {
    pub calendars: Vec<Calendar>,
    pub ui: UIState,
    pub selection: Option<(DateTime<Local>, DateTime<Local>)>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            calendars: vec![Calendar { events: Vec::new() }],
            selection: None,
            ui: UIState {
                mobile: false,
                navigation_history: vec!["month".into()],
            },
        }
    }
}

// Define the static STORE variable based on Action and State definitions.
// ------------------------------------------------------------------------
store!(Action, State);

// Selectors
// ------------------------------------------------------------------------

pub fn select_ui_state(callback: impl Fn(&UIState) + 'static) {
    store().select(move |_, state| callback(&state.ui));
}

pub fn select_selection(callback: impl Fn(&Option<(DateTime<Local>, DateTime<Local>)>) + 'static) {
    store().select(move |_, state| callback(&state.selection));
}

pub fn select_current_view_and_selection(
    callback: impl Fn(&String, &Option<(DateTime<Local>, DateTime<Local>)>) + 'static,
) {
    store().select(move |_, state| {
        callback(
            &state.ui.navigation_history.last().unwrap(),
            &state.selection,
        )
    });
}

pub fn select_calendars(callback: impl Fn(&Vec<Calendar>) + 'static) {
    store().select(move |_, state| callback(&state.calendars));
}
