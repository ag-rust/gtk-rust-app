use gtk::prelude::*;
use libadwaita as adw;

pub const SIDEBAR_WIDTH: i32 = 260;
pub const CONTENT_WIDTH: i32 = 480;

pub fn header(sidebar_size_group: &gtk::SizeGroup, content_size_group: &gtk::SizeGroup) -> adw::Leaflet {
    let leaflet = adw::Leaflet::builder()
        .transition_type(adw::LeafletTransitionType::Slide)
        .build();

    let sidebar = sidebar(&leaflet);
    sidebar_size_group.add_widget(&sidebar);
    let content = content(&leaflet);
    content_size_group.add_widget(&content);

    leaflet.append(&sidebar);
    let page = leaflet.page(&sidebar).unwrap();
    page.set_name(Some("sidebar"));

    leaflet.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    leaflet.append(&content);
    let page = leaflet.page(&content).unwrap();
    page.set_name(Some("content"));

    leaflet.set_visible_child_name("content");

    leaflet
}

fn sidebar(leaflet: &adw::Leaflet) -> adw::HeaderBar {
    let header_bar = adw::HeaderBar::builder()
        .width_request(SIDEBAR_WIDTH)
        .build();
    header_bar.set_show_end_title_buttons(false);
    let forward_button = gtk::Button::from_icon_name(Some("go-next-symbolic"));
    leaflet.bind_property("folded", &forward_button, "visible").build();
    forward_button.connect_clicked(glib::clone!(@weak leaflet => move |_| {
        leaflet.set_visible_child_name("content");
    }));
    header_bar.pack_end(&forward_button);
    header_bar
}

fn content(leaflet: &adw::Leaflet) -> adw::HeaderBar {
    let hb = adw::HeaderBar::builder()
        .width_request(CONTENT_WIDTH)
        .hexpand(true)
        .build();
    leaflet
        .bind_property("folded", &hb, "show_start_title_buttons")
        .build();
    hb
}
