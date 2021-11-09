use gtk::prelude::*;
use libadwaita as adw;

use super::menu_button;

pub fn leaflet_header(
    header_sidebar: &adw::HeaderBar,
    header_content: &adw::HeaderBar,
) -> adw::Leaflet {
    let leaflet = adw::Leaflet::builder()
        .transition_type(adw::LeafletTransitionType::Slide)
        .build();

    leaflet.append(header_sidebar);
    let page = leaflet.page(header_sidebar).unwrap();
    page.set_name(Some("sidebar"));

    leaflet.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    leaflet.append(header_content);
    let page = leaflet.page(header_content).unwrap();
    page.set_name(Some("content"));

    leaflet.set_visible_child_name("content");

    leaflet
}

pub fn header_sidebar(settings: &Option<gdk4::gio::Settings>) -> adw::HeaderBar {
    let header_bar = adw::HeaderBar::builder()
        .title_widget(&gtk::Label::new(None))
        .build();

    if let Some(settings) = settings {
        settings
            .bind("sidebar-width-request", &header_bar, "width-request")
            .build();
    }

    header_bar.set_show_end_title_buttons(false);
    let forward_button = gtk::Button::from_icon_name(Some("go-next-symbolic"));
    forward_button.set_visible(false);

    header_bar
}

pub fn header_main(settings: &Option<gdk4::gio::Settings>) -> adw::HeaderBar {
    let header_bar = adw::HeaderBar::builder().hexpand(true).build();

    if let Some(settings) = settings {
        settings
            .bind("sidebar-width-request", &header_bar, "width-request")
            .build();
    }

    let b = menu_button();

    header_bar.pack_end(&b);

    header_bar
}
