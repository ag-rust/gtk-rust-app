use gtk::prelude::*;
// use libadwaita as adw;

pub fn view2() -> gtk::Box {
    let view = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    view.append(&gtk::Label::new(Some("View 2")));
    view
}
