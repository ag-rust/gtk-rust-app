use glib::ObjectExt;
use gtk::prelude::*;
use libadwaita as adw;

#[derive(Debug, Clone)]
pub struct View {
    pub start_header_widgets: Vec<gtk::Widget>,
    pub end_header_widgets: Vec<gtk::Widget>,
    pub widget: gtk::Widget,
    pub name: &'static str,
    pub title_and_icon: Option<(String, String)>,
}

use crate::ui::components;

pub fn window(
    package_name: &String,
    title: &Option<String>,

    app: &gtk::Application,
    views: &Vec<View>,

    header_sidebar: &Option<adw::HeaderBar>,
    header_main: &Option<adw::HeaderBar>,

    sidebar: &Option<Box<dyn Fn(&adw::ViewStack) -> gtk::Widget + 'static>>,
    view_switcher_bar: &Option<Box<dyn Fn(&adw::ViewStack) -> adw::ViewSwitcherBar + 'static>>,

    settings: &Option<gdk4::gio::Settings>,

    connect_leaflet: &Option<Box<dyn Fn(&adw::Leaflet) + 'static>>,
    connect_view_stack: &Option<Box<dyn Fn(&adw::ViewStack) + 'static>>,
) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title(title.as_ref().unwrap_or(package_name))
        .build();

    if let Some(settings) = settings {
        let w = settings.get("window-width");
        let h = settings.get("window-height");

        window.set_default_width(w);
        window.set_default_height(h);

        let settings = settings.clone();
        window.connect_close_request(move |win| {
            let width = win.width();
            let height = win.height();
            settings.set_int("window-width", width - 122).unwrap();
            settings.set_int("window-height", height - 122).unwrap();
            gtk::Inhibit(false)
        });
    }

    let sidebar_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);
    let main_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);

    let content_leaflet = adw::Leaflet::builder()
        .transition_type(adw::LeafletTransitionType::Slide)
        .build();
    if let Some(cl) = connect_leaflet {
        cl(&content_leaflet);
    }
    let main_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    main_size_group.add_widget(&main_box);

    let view_stack = view_stack(views, settings);
    if let Some(cvs) = connect_view_stack {
        cvs(&view_stack);
    }

    main_box.append(&view_stack);

    let view_switcher = if let Some(vsb) = view_switcher_bar {
        vsb(&view_stack)
    } else {
        components::view_switcher_bar(&view_stack)
    };

    content_leaflet
        .bind_property("folded", &view_switcher, "reveal")
        .build();

    main_box.append(&view_switcher);
    if let Some(sidebar) = sidebar {
        let sidebar = sidebar(&view_stack);
        sidebar_size_group.add_widget(&sidebar);
        content_leaflet.append(&sidebar);
        let page = content_leaflet.page(&sidebar).unwrap();
        page.set_name(Some("sidebar"));
        content_leaflet.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    }
    content_leaflet.append(&main_box);
    let page = content_leaflet.page(&main_box).unwrap();
    page.set_name(Some("content"));
    content_leaflet.set_visible_child_name("content");
    window.set_child(Some(&content_leaflet));

    if let Some(header_main) = header_main {
        for view in views {
            // let name = view.name.unwrap();
            for w in &view.start_header_widgets {
                header_main.pack_start(w);
            }
            for w in &view.end_header_widgets {
                header_main.pack_end(w);
            }
        }
        main_size_group.add_widget(header_main);
        if let Some(header_sidebar) = header_sidebar {
            sidebar_size_group.add_widget(header_sidebar);
            let header_leaflet = components::leaflet_header(&header_sidebar, &header_main);
            content_leaflet
                .bind_property("visible_child_name", &header_leaflet, "visible_child_name")
                .build();
            window.set_titlebar(Some(&header_leaflet));
        } else {
            window.set_titlebar(Some(header_main));
        }
    }

    return window;
}

fn view_stack(views: &Vec<View>, settings: &Option<gdk4::gio::Settings>) -> adw::ViewStack {
    let stack = adw::ViewStack::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    if let Some(settings) = settings {
        settings
            .bind("content-width-request", &stack, "width-request")
            .build();
    }

    for view in views {
        if let Some((title, icon)) = &view.title_and_icon {
            stack.add_titled(&view.widget, Some(view.name), title);
            let page = stack.page(&view.widget).unwrap();
            page.set_icon_name(Some(icon));
        } else {
            stack.add_named(&view.widget, Some(view.name));
        }
    }

    stack
}
