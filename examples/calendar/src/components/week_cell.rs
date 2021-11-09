use gtk::prelude::*;

pub fn week_cell(hour: u32) -> gtk::Box {
    let cell = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .width_request(40)
        .height_request(20)
        .vexpand(true)
        .build();
    cell.style_context().add_class("week-cell");
    if hour % 2 != 0 {
        cell.style_context().add_class("week-cell-full");
    }
    cell
}

pub fn week_cell_header(label: &str) -> gtk::Box {
    let cell = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .width_request(40)
        .height_request(20)
        .vexpand(true)
        .build();
    cell.style_context().add_class("week-cell");
    cell.style_context().add_class("week-cell-header");
    if label.contains(":") {
        cell.style_context().add_class("week-cell-header-left");
    } else {
        cell.style_context().add_class("week-cell-header-top");
    }
    cell.append(
        &gtk::Label::builder()
            .label(label)
            .ellipsize(gdk4::pango::EllipsizeMode::End)
            .build(),
    );
    cell
}
