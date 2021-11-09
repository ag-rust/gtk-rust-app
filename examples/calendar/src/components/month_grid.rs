use chrono::prelude::*;
use gtk::prelude::*;

use crate::components::{month_cell, month_headercell};

state! {
    [selection, set_selection]: Option<Date<Local>> = None,
}

pub fn month_grid(
    delta_month: i32,
    select_day: &'static impl Fn(&gtk::Box, Option<Date<Local>>),
) -> gtk::Box {
    let month_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .vexpand(true)
        .hexpand(true)
        .spacing(8)
        .build();

    let today_month = Local::today();

    // month for this grid
    let this_month = get_month_by_delta(&today_month, delta_month);
    let this_month_first_day = this_month.with_day0(0).unwrap();

    month_box.set_widget_name(&format!("month-{}", this_month.month()));

    let title = this_month_first_day.format("%Y - %m").to_string();
    let title_label = gtk::Label::builder().label(&title).build();
    title_label.style_context().add_class("month-title");
    month_box.append(&title_label);

    let month_grid = gtk::Grid::builder()
        .hexpand(true)
        .vexpand(true)
        .column_homogeneous(true)
        .build();
    month_grid.style_context().add_class("month-grid");

    for i in 0..7 {
        month_grid.attach(&month_headercell(i), i as i32, 0, 1, 1);
    }

    let former_month = get_month_by_delta(&this_month, -1);
    let former_month_len = get_month_len(former_month);
    let next_month = get_month_by_delta(&this_month, 1);
    let this_month_len = get_month_len(this_month);
    let this_month_start = this_month_first_day.weekday().number_from_monday() - 1;
    for week in 0..6 {
        for day in 0..7 {
            let day_num = day + (week * 7);

            let (date, other) = if day_num < this_month_start {
                (
                    former_month
                        .with_day0(former_month_len - (this_month_start - day_num))
                        .unwrap(),
                    true,
                )
            } else if day_num - this_month_start >= this_month_len {
                (
                    next_month
                        .with_day0(day_num - this_month_start - this_month_len)
                        .unwrap(),
                    true,
                )
            } else {
                (
                    this_month.with_day0(day_num - this_month_start).unwrap(),
                    false,
                )
            };

            let d = date.clone();
            let cell = month_cell(date.clone(), other, move || set_selection(Some(d)));

            bind_state(
                "selection",
                glib::clone!(@weak cell, @weak month_box => move || {
                    if Some(date) == *selection() {
                        if cell.style_context().has_class("month-cell-selected") {
                            cell.style_context().remove_class("month-cell-selected");
                            select_day(&month_box, None);
                        } else {
                            cell.style_context().add_class("month-cell-selected");
                            select_day(&month_box, Some(date));
                        }
                    } else {
                        cell.style_context().remove_class("month-cell-selected");
                    }
                }),
            );

            month_grid.attach(&cell, day as i32, 1 + week as i32, 1, 1);
        }
    }

    month_box.append(&month_grid);

    month_box
}

fn get_month_by_delta(today_month: &Date<Local>, month_delta: i32) -> Date<Local> {
    let month0 = today_month.month0() as i32;
    let year = today_month.year();

    let target_month;
    let target_year;
    if month0 + month_delta >= 0 {
        target_month = (month0 + month_delta) % 12;
        target_year = year + ((month0 + month_delta) / 12);
    } else {
        target_month = 12 - ((month0 + month_delta) % 12).abs();
        target_year = year - ((month0 - 12 + month_delta) / 12).abs();
    }
    today_month
        .with_day0(0)
        .unwrap()
        .with_month0(target_month as u32)
        .unwrap()
        .with_year(target_year)
        .unwrap()
}

fn get_month_len(date: Date<Local>) -> u32 {
    let year: i32 = date.year();
    let month: u32 = date.month();
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days() as u32
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    use super::get_month_by_delta;

    #[test]
    fn test_get_month_by_delta() {
        let today_month: Date<Local> = Local.ymd(2021, 11, 1);

        let result = get_month_by_delta(&today_month, 0);
        assert_eq!(result.month(), 11);
        assert_eq!(result.year(), 2021);

        let result = get_month_by_delta(&today_month, -1);
        assert_eq!(result.month(), 10);
        assert_eq!(result.year(), 2021);

        let result = get_month_by_delta(&today_month, -10);
        assert_eq!(result.month(), 1);
        assert_eq!(result.year(), 2021);

        let result = get_month_by_delta(&today_month, -11);
        assert_eq!(result.month(), 12);
        assert_eq!(result.year(), 2020);

        let result = get_month_by_delta(&today_month, -23);
        assert_eq!(result.month(), 12);
        assert_eq!(result.year(), 2019);

        let result = get_month_by_delta(&today_month, 1);
        assert_eq!(result.month(), 12);
        assert_eq!(result.year(), 2021);

        let result = get_month_by_delta(&today_month, 2);
        assert_eq!(result.month(), 1);
        assert_eq!(result.year(), 2022);

        let result = get_month_by_delta(&today_month, 14);
        assert_eq!(result.month(), 1);
        assert_eq!(result.year(), 2023);
    }
}
