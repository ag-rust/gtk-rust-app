
use gtk::prelude::*;
use libadwaita as adw;

use crate::ui::components;
use crate::ui::views;

use super::components::CONTENT_WIDTH;

const APP_NAME: &str = "Simple App";

pub fn window(app: &adw::Application) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .default_width(600)
        .default_height(900)
        .title(APP_NAME)
        .build();

    let sidebar_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);
    let content_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);

    let header = components::header(&sidebar_size_group, &content_size_group);

    window.set_titlebar(Some(&header));

    let window_content = window_content(&header, &sidebar_size_group, &content_size_group);
    window.set_child(Some(&window_content));

    return window;
}

fn window_content(
    header_leaflet: &adw::Leaflet,
    sidebar_size_group: &gtk::SizeGroup,
    content_size_group: &gtk::SizeGroup,
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
        
    let content_stack = content();
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

fn content() -> adw::ViewStack {
    let content_stack = adw::ViewStack::builder()
        .hexpand(true)
        .vexpand(true)
        .width_request(CONTENT_WIDTH)
        .build();
    let home = views::home();
    let view2 = views::view2();
    let view3 = views::view3();
    content_stack.add_titled(&home, Some("home"), "home");
    content_stack.add_titled(&view2, Some("view 2"), "view2");
    content_stack.add_titled(&view3, Some("view 3"), "view3");
    content_stack
        .page(&home)
        .unwrap()
        .set_icon_name(Some("go-home-symbolic"));
    content_stack
        .page(&view2)
        .unwrap()
        .set_icon_name(Some("system-switch-user-symbolic"));
    content_stack
        .page(&view3)
        .unwrap()
        .set_icon_name(Some("keyboard-layout-filled-symbolic"));
    content_stack.set_visible_child(&home);
    content_stack
}
