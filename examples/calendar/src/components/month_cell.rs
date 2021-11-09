use chrono::prelude::*;
use gettextrs::gettext;
use gtk::{prelude::*, GestureClick};

use crate::{components::event_tag, store::select_calendars};

pub fn month_cell(date: Date<Local>, other: bool, on_click: impl Fn() + 'static) -> gtk::Widget {
    let day_label = gtk::Label::builder()
        .label(&format!("{}", date.day()))
        .valign(gtk::Align::Start)
        .halign(gtk::Align::Center)
        .hexpand(true)
        .build();
    day_label.style_context().add_class("day-number");

    let day_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .width_request(40)
        .height_request(80)
        .vexpand(true)
        .build();
    day_box.style_context().add_class("month-cell");

    day_box.append(&day_label);

    let gesture_click = GestureClick::default();
    day_box.add_controller(&gesture_click);

    gesture_click.connect_end(move |_, _| {
        on_click();
    });

    if other {
        day_box.style_context().add_class("month-cell-other");
    }

    if date == Local::today() {
        day_box.style_context().add_class("month-cell-current");
    }

    let event_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(2)
        .hexpand(true)
        .vexpand(true)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Fill)
        .build();
    day_box.append(&event_box);

    select_calendars(glib::clone!(@weak event_box => move |calendars| {
        let mut child = event_box.first_child();
        while child.is_some() {
            let c = child.unwrap();
            let next = c.next_sibling();
            event_box.remove(&c);
            child = next;
        }
        let mut events = Vec::new();
        for calendar in calendars {
            for event in &calendar.events {
                if event.start.month() == date.month() && event.start.year() == date.year() {
                    if event.start.day() <= date.day() && event.end.day() >= date.day() {
                        events.push(event)    
                    }
                }
            }
        }
        if events.len() <= 2 {
            for event in events {
                event_box.append(&event_tag(&event.name));
            }
        } else if events.len() > 2 {
            event_box.append(&event_tag("..."));
        }

    }));

    day_box.upcast()
}

pub fn month_headercell(day0: i32) -> gtk::Box {
    let names = vec![
        (gettext("Monday"), gettext("Mo")),
        (gettext("Tuesday"), gettext("Tu")),
        (gettext("Wednesday"), gettext("We")),
        (gettext("Thursday"), gettext("Th")),
        (gettext("Friday"), gettext("Fr")),
        (gettext("Saturday"), gettext("Sa")),
        (gettext("Sunday"), gettext("Su")),
    ];

    let day_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    day_box.style_context().add_class("month-cell-header");
    let label = gtk::Label::builder()
        .label(&names[day0 as usize].0)
        .ellipsize(gdk4::pango::EllipsizeMode::End)
        .build();

    crate::store::select_ui_state(glib::clone!(@weak label => move |ui| {
        if ui.mobile {
            label.set_label(&names[day0 as usize].1)
        } else {
            label.set_label(&names[day0 as usize].0)
        }
    }));

    day_box.append(&label);
    day_box
}
