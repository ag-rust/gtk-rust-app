use gtk::prelude::*;
use gettextrs::gettext;

use crate::components::{week_cell, week_cell_header};

pub fn week_grid(_delta_week: i32) -> gtk::Widget {
    let week_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .build();
    {
        let scroll_view = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .build();

        let week_grid = gtk::Grid::builder()
            .hexpand(true)
            .vexpand(true)
            .column_homogeneous(true)
            .build();

        week_grid.style_context().add_class("month-grid");

        week_grid.attach(&week_cell_header(&gettext("Monday")), 1, 0, 1, 1);
        week_grid.attach(&week_cell_header(&gettext("Tuesday")), 2, 0, 1, 1);
        week_grid.attach(&week_cell_header(&gettext("Wednesday")), 3, 0, 1, 1);
        week_grid.attach(&week_cell_header(&gettext("Thursday")), 4, 0, 1, 1);
        week_grid.attach(&week_cell_header(&gettext("Friday")), 5, 0, 1, 1);
        week_grid.attach(&week_cell_header(&gettext("Saturday")), 6, 0, 1, 1);
        week_grid.attach(&week_cell_header(&gettext("Sunday")), 7, 0, 1, 1);

        for h in 0..48 {
            if h % 2 == 0 {
                week_grid.attach(
                    &week_cell_header(&format!("{:#02}:00", (h / 2))),
                    0,
                    1 + h,
                    1,
                    1,
                );
            }
            for d in 0..7 {
                week_grid.attach(&week_cell(h as u32), 1 + d, 1 + h, 1, 1);
            }
        }

        scroll_view.set_child(Some(&week_grid));
        week_box.append(&scroll_view)
    }
    week_box.upcast()
}
