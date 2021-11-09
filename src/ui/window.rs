use gdk4::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::prelude::WidgetExt;
use libadwaita as adw;

use crate::{
    ui::{components, View},
    ProjectDescriptor,
};

pub fn window(
    project_descriptor: ProjectDescriptor,
    app: adw::Application,
    views: Vec<View>,
    settings: gdk4::gio::Settings,
) -> adw::ApplicationWindow {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title(&project_descriptor.package.name)
        .build();

    let sidebar_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);
    let content_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);

    let header = components::header(&sidebar_size_group, &content_size_group);

    window.set_titlebar(Some(&header));

    let window_content = window_content(&header, &sidebar_size_group, &content_size_group, views);
    window.set_child(Some(&window_content));

    return window;
}

fn window_content(
    header_leaflet: &adw::Leaflet,
    sidebar_size_group: &gtk::SizeGroup,
    content_size_group: &gtk::SizeGroup,
    views: Vec<View>,
) -> adw::Leaflet {
    let content_leaflet = adw::Leaflet::builder()
        .transition_type(adw::LeafletTransitionType::Slide)
        .build();

    header_leaflet
        .bind_property("visible_child_name", &content_leaflet, "visible_child_name")
        .build();

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let content_stack = content(views);
    content_box.append(&content_stack);
    content_size_group.add_widget(&content_stack);

    let view_switcher = adw::ViewSwitcherBar::builder()
        .stack(&content_stack)
        .build();

    content_leaflet
        .bind_property("folded", &view_switcher, "reveal")
        .build();
        
    content_box.append(&view_switcher);

    let sidebar = views::sidebar(&content_stack);
    sidebar_size_group.add_widget(&sidebar);

    content_leaflet.append(&sidebar);
    let page = content_leaflet.page(&sidebar).unwrap();
    page.set_name(Some("sidebar"));

    content_leaflet.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    content_leaflet.append(&content_box);
    let page = content_leaflet.page(&content_box).unwrap();
    page.set_name(Some("content"));

    content_leaflet.set_visible_child_name("content");

    content_leaflet
}

fn content(views: Vec<View>) -> adw::ViewStack {
    let content_stack = adw::ViewStack::builder()
        .hexpand(true)
        .vexpand(true)
        .width_request(CONTENT_WIDTH)
        .build();
    for view in views {
        content_stack.add_titled(&view.widget, view.name, view.title);
        let page = content_stack.page(&view.widget).unwrap()
        page.set_icon_name(view.icon_name);
    }
    content_stack.set_visible_child(&home);
    content_stack
}
