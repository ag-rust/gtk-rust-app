use adw::prelude::ActionRowExt;
use gtk::prelude::*;
use libadwaita as adw;

use crate::ui::components::SIDEBAR_WIDTH;

pub fn sidebar(content_stack: &adw::ViewStack) -> gtk::ScrolledWindow {
    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .width_request(SIDEBAR_WIDTH)
        .build();
    let sidebar = gtk::ListBox::builder().build();
    scrolled_window.set_child(Some(&sidebar));
    let model = content_stack.pages().unwrap();
    for i in 0..model.n_items() {
        let o = model.item(i).unwrap();
        let page: adw::ViewStackPage = o.downcast().unwrap();

        let name = page.name().map(|n| n.to_string()).unwrap_or("".into());

        let entry = gtk::Box::builder().spacing(8).build();
        entry.style_context().add_class("navigation-entry");
        entry.append(&gtk::Image::from_icon_name(Some(
            &page
                .icon_name()
                .map(|n| n.to_string())
                .unwrap_or("".into())
                .as_str(),
        )));

        entry.append(&gtk::Label::new(Some(
            &page
                .name()
                .map(|n| n.to_string())
                .unwrap_or("".into())
                .as_str(),
        )));

        let row = adw::ActionRow::builder()
            .selectable(true)
            .activatable(true)
            .child(&entry)
            .build();
        row.connect_activated(glib::clone!( @weak content_stack => move |_| {
            content_stack.set_visible_child_name(&name)
        }));
        sidebar.append(&row);
    }
    scrolled_window
}