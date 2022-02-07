// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::Cell;

use adw::prelude::ActionRowExt;
use gdk4::{gio::Settings, subclass::prelude::ObjectSubclassIsExt};
use gtk::prelude::*;
use gtk_rust_app_derive::widget;
use libadwaita as adw;

/// A Page descripes a view in the main view stack of the leaflet_layout.
///
/// A Page always has an internal name and may define a title and icon.
pub trait Page {
    fn name(&self) -> &'static str;
    fn title_and_icon(&self) -> Option<(String, String)>;
}

#[derive(Debug)]
pub struct PageDesc {
    pub widget: gtk::Widget,
    pub name: &'static str,
    pub title_and_icon: Option<(String, String)>,
}

/// A Header widget describes any widget that is supposed to be shown
/// in the header bar of the leaflet_layout.
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
    pub fn start(widget: impl IsA<gtk::Widget>) -> Self {
        HeaderWidget {
            widget: widget.upcast(),
            alignment: HeaderAlignment::Start,
        }
    }
    pub fn end(widget: impl IsA<gtk::Widget>) -> Self {
        HeaderWidget {
            widget: widget.upcast(),
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
#[widget(gtk::Box)]
#[template(file = "leaflet_layout.xml")]
pub struct LeafletLayout {
    #[template_child]
    pub leaflet: TemplateChild<adw::Leaflet>,
    #[template_child]
    pub sidebar_header: TemplateChild<adw::HeaderBar>,
    #[template_child]
    pub main_header: TemplateChild<adw::HeaderBar>,
    #[template_child]
    pub view_stack: TemplateChild<adw::ViewStack>,
    #[template_child]
    pub navigation_sidebar: TemplateChild<gtk::ListBox>,
    #[template_child]
    pub sidebar: TemplateChild<gtk::Box>,
    #[template_child]
    pub sidebar_content: TemplateChild<adw::Leaflet>,
    #[template_child]
    pub sidebar_scrolled_window: TemplateChild<gtk::ScrolledWindow>,
    #[template_child]
    pub main: TemplateChild<gtk::Box>,
    #[template_child]
    pub view_switcher_bar: TemplateChild<adw::ViewSwitcherBar>,
    #[template_child]
    pub toast_overlay: TemplateChild<adw::ToastOverlay>,

    #[property_bool]
    pub mobile: Cell<bool>,

    #[signal]
    adapt: (),
}

impl LeafletLayout {
    pub fn new(
        settings: Option<&gdk4::gio::Settings>,
        sidebar_header_widgets: Vec<HeaderWidget>,
        main_header_widgets: Vec<HeaderWidget>,
        pages: Vec<PageDesc>,
    ) -> Self {
        let self_: LeafletLayout = glib::Object::new(&[]).expect("Failed to create LeafletLayout");

        if let Some(settings) = settings {
            settings
                .bind("sidebar-width-request", self_.sidebar(), "width-request")
                .build();
            settings
                .bind("main-width-request", self_.main(), "width-request")
                .build();
        }

        for hw in &sidebar_header_widgets {
            match hw.alignment {
                HeaderAlignment::Start => self_.sidebar_header().pack_start(&hw.widget),
                HeaderAlignment::End => self_.sidebar_header().pack_end(&hw.widget),
            }
        }
        for hw in &main_header_widgets {
            match hw.alignment {
                HeaderAlignment::Start => self_.main_header().pack_start(&hw.widget),
                HeaderAlignment::End => self_.main_header().pack_end(&hw.widget),
            }
        }

        for page in pages {
            if let Some((title, icon)) = &page.title_and_icon {
                self_
                    .view_stack()
                    .add_titled(&page.widget, Some(page.name), title);
                let page = self_.view_stack().page(&page.widget);
                page.set_icon_name(Some(icon));
            } else {
                self_.view_stack().add_named(&page.widget, Some(page.name));
            }
        }

        self_
            .leaflet()
            .bind_property("folded", self_.view_switcher_bar(), "reveal")
            .build();

        append_views_to_sidebar(self_.view_stack(), self_.navigation_sidebar());

        self_
    }

    pub fn constructed(&self) {
        let s = self;
        self.imp()
            .leaflet
            .connect_folded_notify(glib::clone!(@weak s => move |l| {
                s.imp().mobile.set(l.is_folded());
                s.emit_adapt()
            }));
    }

    pub fn leaflet(&self) -> &adw::Leaflet {
        &self.imp().leaflet
    }
    pub fn sidebar_header(&self) -> &adw::HeaderBar {
        &self.imp().sidebar_header
    }
    pub fn main_header(&self) -> &adw::HeaderBar {
        &self.imp().main_header
    }
    pub fn view_stack(&self) -> &adw::ViewStack {
        &self.imp().view_stack
    }
    pub fn navigation_sidebar(&self) -> &gtk::ListBox {
        &self.imp().navigation_sidebar
    }
    pub fn sidebar(&self) -> &gtk::Box {
        &self.imp().sidebar
    }
    pub fn sidebar_content(&self) -> &adw::Leaflet {
        &self.imp().sidebar_content
    }
    pub fn sidebar_scrolled_window(&self) -> &gtk::ScrolledWindow {
        &self.imp().sidebar_scrolled_window
    }
    pub fn main(&self) -> &gtk::Box {
        &self.imp().main
    }
    pub fn view_switcher_bar(&self) -> &adw::ViewSwitcherBar {
        &self.imp().view_switcher_bar
    }
    pub fn toast_overlay(&self) -> &adw::ToastOverlay {
        &self.imp().toast_overlay
    }

    pub fn is_mobile(&self) -> bool {
        let mobile = self.imp().mobile.take();
        self.imp().mobile.set(mobile);
        mobile
    }

    pub fn connect_adapt(&self, f: impl Fn(&Self) + 'static) {
        self._connect_adapt(f);
    }

    pub fn show_message(&self, msg: &str) -> adw::Toast {
        let toast = adw::Toast::new(msg);
        self.toast_overlay().add_toast(&toast);
        toast
    }

    pub fn show_toast(&self, toast: &adw::Toast) {
        self.toast_overlay().add_toast(toast);
    }

    pub fn builder(settings: Option<&Settings>) -> LeafletLayoutBuilder {
        LeafletLayoutBuilder::new(settings)
    }
}

fn append_views_to_sidebar(view_stack: &adw::ViewStack, navigation_sidebar: &gtk::ListBox) {
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

            navigation_sidebar.append(&row);
        }
    }
}

pub struct LeafletLayoutBuilder<'a> {
    settings: Option<&'a gdk4::gio::Settings>,
    sidebar_header_widgets: Vec<HeaderWidget>,
    main_header_widgets: Vec<HeaderWidget>,
    pages: Vec<PageDesc>,
}
impl<'a> LeafletLayoutBuilder<'a> {
    pub fn new(settings: Option<&'a Settings>) -> Self {
        Self {
            settings,
            sidebar_header_widgets: Vec::new(),
            main_header_widgets: Vec::new(),
            pages: Vec::new(),
        }
    }

    pub fn add_page(self, page: impl Page + IsA<gtk::Widget>) -> Self {
        let mut s = self;
        let name = page.name();
        let title_and_icon = page.title_and_icon();
        let page_desc = PageDesc {
            widget: page.upcast(),
            name,
            title_and_icon,
        };
        s.pages.push(page_desc);
        s
    }

    pub fn add_main_header_start(self, widget: impl IsA<gtk::Widget>) -> Self {
        let mut s = self;
        s.main_header_widgets.push(HeaderWidget {
            widget: widget.upcast(),
            alignment: HeaderAlignment::Start,
        });
        s
    }

    pub fn add_main_header_end(self, widget: impl IsA<gtk::Widget>) -> Self {
        let mut s = self;
        s.main_header_widgets.push(HeaderWidget {
            widget: widget.upcast(),
            alignment: HeaderAlignment::End,
        });
        s
    }

    pub fn build(self) -> LeafletLayout {
        LeafletLayout::new(
            self.settings,
            self.sidebar_header_widgets,
            self.main_header_widgets,
            self.pages,
        )
    }
}
