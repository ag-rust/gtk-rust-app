use gtk::prelude::*;
// use libadwaita as adw;

pub fn home() -> gtk::Box {
    let screen = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    screen.append(&gtk::Label::new(Some("Home")));
    screen
}
