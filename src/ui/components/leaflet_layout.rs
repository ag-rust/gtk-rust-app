// SPDX-License-Identifier: GPL-3.0-or-later

use adw::prelude::ActionRowExt;
use gtk::prelude::*;
use libadwaita as adw;

use crate::__interface;

/// A Page descripes a view in the main view stack of the leaflet_layout.
///
/// A Page always has an internal name and may define a title and icon.
///
/// # Example
/// ```rust,no_run
/// use gtk::prelude::*;
/// use gtk_rust_app::components::leaflet_layout::Page;
///
/// Page::new(
///     gtk::Box::builder().build(),
///     name: "home",
///     title_and_icon: Some(("Home".to_string(), "go-home-symbolic".to_string()))
/// )
/// ```
#[derive(Debug, Clone)]
pub struct Page {
    widget: gtk::Widget,
    name: &'static str,
    title_and_icon: Option<(String, String)>,
}

impl Page {
    pub fn new(
        widget: gtk::Widget,
        name: &'static str,
        title_and_icon: Option<(String, String)>,
    ) -> Self {
        Page {
            widget,
            name,
            title_and_icon,
        }
    }
}

/// A Header widget describes any widget that is supposed to be shown
/// in the header bar of the leaflet_layout.
///
/// # Examples
///
/// ```rust
/// use gtk::prelude::*;
/// use gtk_rust_app::components::leaflet_layout::HeaderWidget;
///
/// // Place a button on the left side of the header bar
/// let hw = HeaderWidget::start(gtk::Button::with_label("Test"));
/// // Place a button on the right side of the header bar
/// let hw = HeaderWidget::end(gtk::Button::with_label("Test"));
/// ```
#[derive(Debug, Clone)]
pub struct HeaderWidget {
    widget: gtk::Widget,
    alignment: HeaderAlignment,
}

/// Describes where to place a HeaderWidget in a gtk HeaderBar.
#[derive(Debug, Clone)]
enum HeaderAlignment {
    Start,
    End,
}
impl HeaderWidget {
    pub fn start(widget: gtk::Widget) -> Self {
        HeaderWidget {
            widget,
            alignment: HeaderAlignment::Start,
        }
    }
    pub fn end(widget: gtk::Widget) -> Self {
        HeaderWidget {
            widget,
            alignment: HeaderAlignment::End,
        }
    }
}

/// A Basic app layout which is based on a libadwaita leaflet widget.
///
/// This layout uses a AdwLeaflet to build a sidebar on the left and a main content area.
///
///
/// ```txt
/// ┌────────┬──────────────────────┐
/// │        │                  -xo │
/// ├────────┼──────────────────────┤
/// │ Home   │                      │
/// │        │                      │
/// │ Page1  │                      │
/// │        │                      │
/// │ Page2  │                      │
/// │        │                      │
/// │        │                      │
/// │        │                      │
/// │        │                      │
/// │        │                      │
/// └────────┴──────────────────────┘    
/// ```
///
/// The main content shows a vector of widgets views of a AdwViewStack. Those main views are called pages.
///
/// - The sidebar is hidden when the window size shrinks to a threashold.
/// - A AdwViewSwitcherBar appears on the bottom to allow mobile navigation.
///
/// ```txt
/// ┌───────────┐
/// │       -xo │
/// ├───────────┤
/// │           │
/// │           │
/// │           │
/// │           │
/// │           │
/// │           │
/// │           │
/// │           │
/// ├───────────┤
/// │ H  P1  P2 │
/// └───────────┘
/// ```
///
pub struct LeafletLayout {
    pub leaflet: adw::Leaflet,
    pub sidebar_header: adw::HeaderBar,
    pub main_header: adw::HeaderBar,
    pub view_stack: adw::ViewStack,
    pub navigation_sidebar: gtk::ListBox,
    pub sidebar: gtk::Box,
    pub sidebar_scrolled_window: gtk::ScrolledWindow,
    pub main: gtk::Box,
    pub view_switcher_bar: adw::ViewSwitcherBar,
}

pub fn leaflet_layout(
    settings: Option<&gdk4::gio::Settings>,
    sidebar_header_widgets: Vec<HeaderWidget>,
    main_header_widgets: Vec<HeaderWidget>,
    views: Vec<Page>,
) -> LeafletLayout {
    __interface!(crate::regex::Regex,
    r#"
    <object class="AdwLeaflet" id="leaflet">
        <property name="visible-child">main</property>
        <property name="fold-threshold-policy">ADW_FOLD_THRESHOLD_POLICY_NATURAL</property>

        <child>
        <object class="GtkBox" id="sidebar">
            <property name="orientation">vertical</property>
            <property name="width-request">100</property>
            <child>
            <object class="AdwHeaderBar" id="sidebar_header">>
                <property name="show-end-title-buttons">False</property>
                <property name="title-widget">
                    <object class="GtkLabel"></object>
                </property>
            </object>
            </child>
            <child>
            <object class="GtkScrolledWindow" id="sidebar_scrolled_window">
                <property name="hscrollbar-policy">never</property>
                <property name="hexpand">False</property>
                <property name="vexpand">True</property>
                <child>
                <object class="GtkViewport">
                    <child>
                    <object class="GtkListBox" id="navigation_sidebar">
                        <style>
                            <class name="navigation-sidebar" />
                        </style>
                    </object>
                    </child>
                </object>
                </child>
            </object>
            </child>
        </object>
        </child>

        
        <child>
        <object class="GtkSeparator">
            <property name="orientation">horizontal</property>
        </object>
        </child>

        <child>
        <object class="GtkBox" id="main">
            <property name="orientation">vertical</property>
            <property name="width-request">300</property>
            <child>
            <object class="AdwHeaderBar" id="main_header">
                <property name="hexpand">True</property>
            </object>
            </child>
            <child>
            <object class="GtkBox" id="main_content">
                <property name="hexpand">True</property>
                <property name="vexpand">True</property>
                <property name="orientation">vertical</property>
                <child>
                    <object class="AdwViewStack" id="view_stack">
                        <property name="hexpand">True</property>
                        <property name="vexpand">True</property>
                        <property name="hhomogeneous">False</property>
                    </object>
                </child>
            
            </object>
            </child>
            <child>
            <object class="AdwViewSwitcherBar" id="view_switcher_bar">
                <property name="stack">view_stack</property>
            </object>
            </child>
        </object>
        </child>


    </object>
    
    <object class="GtkSizeGroup" id="main_size_group">
        <widgets>
            <widget name="main_header"/>
            <widget name="main_content"/>
        </widgets>
    </object>
    <object class="GtkSizeGroup" id="sidebar_size_group">
        <widgets>
            <widget name="sidebar_header"/>
            <widget name="sidebar_scrolled_window"/>
        </widgets>
    </object>
    <object class="GtkSizeGroup" id="headers_size_group">
        <property name="mode">GTK_SIZE_GROUP_VERTICAL</property>
        <widgets>
            <widget name="main_header"/>
            <widget name="sidebar_header"/>
        </widgets>
    </object>
    "#
        leaflet: adw::Leaflet,
        sidebar_header: adw::HeaderBar,
        main_header: adw::HeaderBar,
        view_stack: adw::ViewStack,
        navigation_sidebar: gtk::ListBox,
        sidebar: gtk::Box,
        sidebar_scrolled_window: gtk::ScrolledWindow,
        main: gtk::Box,
        view_switcher_bar: adw::ViewSwitcherBar,
    );

    for hw in &sidebar_header_widgets {
        match hw.alignment {
            HeaderAlignment::Start => sidebar_header.pack_start(&hw.widget),
            HeaderAlignment::End => sidebar_header.pack_end(&hw.widget),
        }
    }
    for hw in &main_header_widgets {
        match hw.alignment {
            HeaderAlignment::Start => main_header.pack_start(&hw.widget),
            HeaderAlignment::End => main_header.pack_end(&hw.widget),
        }
    }

    if let Some(settings) = settings {
        settings
            .bind("sidebar-width-request", &sidebar, "width-request")
            .build();
        settings
            .bind("main-width-request", &main, "width-request")
            .build();
    }

    for view in views {
        if let Some((title, icon)) = &view.title_and_icon {
            view_stack.add_titled(&view.widget, Some(view.name), title);
            let page = view_stack.page(&view.widget).unwrap();
            page.set_icon_name(Some(icon));
        } else {
            view_stack.add_named(&view.widget, Some(view.name));
        }
    }

    leaflet
        .bind_property("folded", &view_switcher_bar, "reveal")
        .build();

    append_views_to_sidebar(&view_stack, &navigation_sidebar);

    LeafletLayout {
        leaflet,
        sidebar_header,
        main_header,
        view_stack,
        navigation_sidebar,
        sidebar_scrolled_window,
        sidebar,
        main,
        view_switcher_bar,
    }
}

fn append_views_to_sidebar(view_stack: &adw::ViewStack, navigation_sidebar: &gtk::ListBox) {
    let model = view_stack.pages().unwrap();
    for i in 0..model.n_items() {
        let o = model.item(i).unwrap();
        let page: adw::ViewStackPage = o.downcast().unwrap();

        let name = page.name().map(|n| n.to_string()).unwrap_or("".into());

        if page.title().is_some() {
            let row = adw::ActionRow::builder()
                .icon_name(&page.icon_name().unwrap_or("".into()))
                .title(&page.title().unwrap_or("".into()))
                .selectable(true)
                .activatable(true)
                .build();

            row.connect_activated(glib::clone!( @weak view_stack => move |_| {
                view_stack.set_visible_child_name(&name)
            }));

            navigation_sidebar.append(&row);
        }
    }
}
