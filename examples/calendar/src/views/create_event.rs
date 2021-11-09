use adw::prelude::ActionRowExt;
use chrono::prelude::*;
use gettextrs::gettext;
use gtk::prelude::*;
use libadwaita as adw;

use crate::store::{select_selection, select_ui_state, Action, CalendarEvent};

state! {
    [name, set_name]: String = "".into(),
    [location, set_location]: String = "".into(),
    [start, set_start]: DateTime<Local> = Local::now(),
    [end, set_end]: DateTime<Local> = Local::now(),
}

pub fn create_event() -> gtk::Widget {
    select_selection(|selection| {
        if let Some((start, end)) = selection {
            set_start(start.clone());
            set_end(end.clone());
        }
    });

    let view = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    view.style_context().add_class("create-event");

    let clamp = adw::Clamp::builder().maximum_size(600).build();

    let scroll_view = gtk::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .build();
    {
        let form_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .spacing(8)
            .build();

        {
            let row_group = gtk::ListBox::builder().build();
            let row_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);
            {
                row_group.style_context().add_class("frame");
                let name_row = entry_row(&gettext("Name"), Some(&row_size_group), |entry| {
                    bind!(
                        state.name: entry.text = name(),
                        connect_changed: |e: &gtk::Entry| e.text().to_string(),
                    );
                });
                row_group.append(&name_row);
                let location_row =
                    entry_row(&gettext("Location"), Some(&row_size_group), |entry| {
                        bind!(
                            state.location: entry.text = location(),
                            connect_changed: |e: &gtk::Entry| e.text().to_string(),
                        );
                    });
                row_group.append(&location_row);
            }
            form_box.append(&row_group);

            form_box.append(&heading(&gettext("Schedule")));

            let row_group = gtk::ListBox::builder().build();
            {
                row_group.style_context().add_class("frame");
                row_group.append(&date_row(&gettext("Start day"), |d, m, y| {
                    bind!(
                        state.start: y.value = start().year(),
                        connect_value_changed: |a: &gtk::Adjustment| start().with_year(a.value() as i32).unwrap_or(start().clone()),
                    );
                    bind!(
                        state.start: m.value = start().month(),
                        connect_value_changed: |a: &gtk::Adjustment| start().with_month(a.value() as u32).unwrap_or(start().clone()),
                    );
                    bind!(
                        state.start: d.value = start().day(),
                        connect_value_changed: |a: &gtk::Adjustment| start().with_day(a.value() as u32).unwrap_or(start().clone()),
                    );
                }));

                row_group.append(&date_row(&gettext("End day"), |d, m, y| {
                    bind!(
                        state.end: y.value = end().year(),
                        connect_value_changed: |a: &gtk::Adjustment| end().with_year(a.value() as i32).unwrap_or(end().clone()),
                    );
                    bind!(
                        state.end: m.value = end().month(),
                        connect_value_changed: |a: &gtk::Adjustment| end().with_month(a.value() as u32).unwrap_or(end().clone()),
                    );
                    bind!(
                        state.end: d.value = end().day(),
                        connect_value_changed: |a: &gtk::Adjustment| end().with_day(a.value() as u32).unwrap_or(end().clone()),
                    );
                }));
            }
            form_box.append(&row_group);

            let save_button = gtk::Button::builder().label(&gettext("Save")).build();
            save_button.connect_clicked(|b| {
                save(&b.upcast_ref());
            });
            select_ui_state(glib::clone!(@weak save_button => move |ui| {
                save_button.set_visible(ui.mobile);
            }));
            form_box.append(&save_button);
        }

        scroll_view.set_child(Some(&form_box));
    }
    clamp.set_child(Some(&scroll_view));
    view.append(&clamp);

    view.upcast()
}

fn row(
    label: &str,
    row_size_group: Option<&gtk::SizeGroup>,
    child: &gtk::Widget,
) -> adw::ActionRow {
    let row = adw::ActionRow::builder()
        .title(label)
        .activatable(false)
        .selectable(false)
        .focusable(false)
        .hexpand(true)
        .build();
    row.style_context().add_class("activatable");
    row.add_suffix(child);
    if let Some(rsg) = row_size_group {
        let inner_box = row.child().unwrap();
        let title = inner_box.last_child().unwrap().prev_sibling().unwrap();
        rsg.add_widget(&title);
    }
    row
}

fn entry_row(
    label: &str,
    row_size_group: Option<&gtk::SizeGroup>,
    connect: impl Fn(&gtk::Entry),
) -> adw::ActionRow {
    let entry = gtk::Entry::builder().hexpand(true).build();
    connect(&entry);
    row(label, row_size_group, &entry.upcast())
}

fn date_row(
    label: &str,
    connect: impl Fn(&gtk::Adjustment, &gtk::Adjustment, &gtk::Adjustment),
) -> adw::ActionRow {
    let month_adjustment = gtk::Adjustment::builder()
        .lower(1.0)
        .upper(12.0)
        .step_increment(1.0)
        .page_increment(5.0)
        .build();
    let day_adjustment = gtk::Adjustment::builder()
        .lower(1.0)
        .upper(31.0)
        .step_increment(1.0)
        .page_increment(5.0)
        .build();
    let year_adjustment = gtk::Adjustment::builder()
        .lower(0.0)
        .upper(9000.0)
        .step_increment(1.0)
        .page_increment(5.0)
        .build();

    let day_spin_button = gtk::SpinButton::builder()
        .orientation(gtk::Orientation::Vertical)
        .adjustment(&day_adjustment)
        .width_request(40)
        .build();
    let month_spin_button = gtk::SpinButton::builder()
        .orientation(gtk::Orientation::Vertical)
        .adjustment(&month_adjustment)
        .width_request(40)
        .build();
    month_spin_button.connect_input(|b| {
        let f = match b.text().as_str() {
            "Jan" => 1 as f64,
            "Feb" => 2 as f64,
            "Mar" => 3 as f64,
            "Apr" => 4 as f64,
            "May" => 5 as f64,
            "Jun" => 6 as f64,
            "Jul" => 7 as f64,
            "Aug" => 8 as f64,
            "Sep" => 9 as f64,
            "Oct" => 10 as f64,
            "Nov" => 11 as f64,
            "Dec" => 12 as f64,
            _ => 0.0,
        };
        Some(Ok(f))
    });
    month_spin_button.connect_output(|b| {
        match b.adjustment().value() as u32 {
            1 => b.set_text("Jan"),
            2 => b.set_text("Feb"),
            3 => b.set_text("Mar"),
            4 => b.set_text("Apr"),
            5 => b.set_text("May"),
            6 => b.set_text("Jun"),
            7 => b.set_text("Jul"),
            8 => b.set_text("Aug"),
            9 => b.set_text("Sep"),
            10 => b.set_text("Oct"),
            11 => b.set_text("Nov"),
            12 => b.set_text("Dec"),
            _ => {}
        };
        gtk::Inhibit(true)
    });
    let year_spin_button = gtk::SpinButton::builder()
        .orientation(gtk::Orientation::Vertical)
        .adjustment(&year_adjustment)
        .width_request(40)
        .build();

    let b = gtk::Box::builder().spacing(16).build();
    let b2 = gtk::Box::builder().spacing(8).build();
    b2.append(&day_spin_button);
    b2.append(&month_spin_button);
    b2.append(&year_spin_button);
    b.append(&b2);

    connect(&day_adjustment, &month_adjustment, &year_adjustment);

    row(label, None, &b.upcast())
}

fn heading(label: &str) -> gtk::Label {
    let label = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .hexpand(true)
        .label(label)
        .build();
    label.style_context().add_class("create-event-heading");
    label
}

pub fn create_event_save_header_button() -> gtk::Widget {
    let button = gtk::Button::builder().label(&gettext("Save")).build();

    select_ui_state(glib::clone!(@weak button => move |ui| {
        button.set_visible(!ui.mobile && ui.navigation_history.last() == Some(&"create-event".to_string()));
    }));

    button.connect_clicked(|b| {
        save(b.upcast_ref());
    });
    button.upcast()
}

fn save(widget: &gtk::Widget) {
    let event = CalendarEvent {
        name: name().clone(),
        location: location().clone(),
        start: start().clone(),
        end: end().clone(),
    };
    init_state();
    dispatch!(widget, Action::EventCreated(event));
    dispatch!(widget, Action::Navigate("back".into()));
}
