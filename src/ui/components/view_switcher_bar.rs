use libadwaita as adw;

pub fn view_switcher_bar(view_stack: &adw::ViewStack) -> adw::ViewSwitcherBar {
    adw::ViewSwitcherBar::builder().stack(view_stack).build()
}