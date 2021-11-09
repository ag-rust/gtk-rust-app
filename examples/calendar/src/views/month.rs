use gtk::prelude::*;

use crate::components::{infinity_carousel, month_grid};

use crate::store::{select_current_view_and_selection, Action};

state!([current_page, set_current_page]: u32 = 0);

pub fn month() -> gtk::Widget {
    let view = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let carousel = infinity_carousel(None, None, |d| {
        month_grid(d, &|widget, date| {
            dispatch!(
                widget,
                Action::Select(date.map(|d| (d.and_hms(0, 0, 0), d.and_hms(0, 0, 0))))
            );
        })
        .upcast()
    });

    carousel.connect_page_changed(move |_carousel, n| unsafe {
        _STATE.current_page = n;
    }); 
    
    // TODO: create connect_state! update_state! macros as alternative to bind! and prevent usage of unsage above

    bind_state(
        "current_page",
        glib::clone!(@weak carousel => move || {
            carousel.scroll_to_full(carousel.nth_page(*current_page()).as_ref().unwrap(), 200);
        }),
    );

    view.append(&carousel);
    view.upcast()
}

pub fn next_month_header_button() -> gtk::Widget {
    let button = gtk::Button::builder()
        .icon_name("go-next-symbolic")
        .visible(false)
        .build();

    select_current_view_and_selection(
        glib::clone!(@weak button => move |current_view, _selection| {
            if current_view == "month" {
                button.set_visible(true)
            } else {
                button.set_visible(false)
            }
        }),
    );

    button.connect_clicked(|_| {
        set_current_page(current_page() + 1);
    });
    button.upcast()
}

pub fn prev_month_header_button() -> gtk::Widget {
    let button = gtk::Button::builder()
        .icon_name("go-previous-symbolic")
        .visible(false)
        .build();

    select_current_view_and_selection(
        glib::clone!(@weak button => move |current_view, _selection| {
            if current_view == "month" {
                button.set_visible(true)
            } else {
                button.set_visible(false)
            }
        }),
    );

    button.connect_clicked(|_| {
        set_current_page(current_page() - 1);
    });
    button.upcast()
}

pub fn open_create_event_view_header_button() -> gtk::Widget {
    let button = gtk::Button::builder()
        .icon_name("list-add-symbolic")
        .visible(false)
        .build();

    select_current_view_and_selection(
        glib::clone!(@weak button => move |current_view, selection| {
            if current_view == "month" {
                button.set_visible(true);
                if selection.is_some() {
                    button.set_sensitive(true);
                } else {
                    button.set_sensitive(false);
                }
            } else {
                button.set_visible(false)
            }
        }),
    );

    button.connect_clicked(|b| dispatch!(b, Action::Navigate("create-event".into())));
    button.upcast()
}
