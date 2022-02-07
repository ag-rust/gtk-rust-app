// SPDX-License-Identifier: GPL-3.0-or-later

use adw::prelude::ActionRowExt;
use gtk::prelude::*;
use libadwaita as adw;

pub fn sidebar(view_stack: &adw::ViewStack) -> gtk::Widget {
    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .build();

    let sidebar = gtk::ListBox::builder().width_request(100).build();
    sidebar.style_context().add_class("navigation-sidebar");
    scrolled_window.set_child(Some(&sidebar));
    let model = view_stack.pages();
    for i in 0..model.n_items() {
        let o = model.item(i).unwrap();
        let page: adw::ViewStackPage = o.downcast().unwrap();

        let name = page
            .name()
            .map(|n| n.to_string())
            .unwrap_or_else(|| "".into());

        if page.title().is_some() {
            let row = adw::ActionRow::builder()
                .icon_name(&page.icon_name().unwrap_or_else(|| "".into()))
                .title(&page.title().unwrap_or_else(|| "".into()))
                .selectable(true)
                .activatable(true)
                .build();

            row.connect_activated(glib::clone!( @weak view_stack => move |_| {
                view_stack.set_visible_child_name(&name)
            }));

            sidebar.append(&row);
        }
    }
    scrolled_window.upcast()
}
