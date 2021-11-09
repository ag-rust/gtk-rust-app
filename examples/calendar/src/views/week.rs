use glib::Cast;
use gtk::prelude::*;

use crate::components::{infinity_carousel, week_grid};

pub fn week() -> gtk::Widget {
    let view = gtk::Box::new(gtk::Orientation::Vertical, 0);
    view.append(&infinity_carousel(None, None, |d| week_grid(d).upcast()));
    view.upcast()
}
