use gtk::prelude::*;
use libadwaita as adw;

state! {
    [current_range, set_current_range]: (i32, i32) = (-2, 2)
}

pub fn infinity_carousel(
    range: Option<(i32, i32)>,
    initial_page: Option<u32>,
    item: impl Fn(i32) -> gtk::Widget + 'static,
) -> adw::Carousel {

    // set defaults
    let range = range.unwrap_or((-2,2));
    let initial_page = initial_page.unwrap_or(2);

    // set initial state
    set_current_range(range);
    
    // define ui
    let carousel = adw::Carousel::builder().build();
    let range = current_range();
    for i in range.0..range.1 + 1 {
        carousel.append(&item(i));
    }
    // workaround to set the initial carousel page to the 3rd page
    carousel.connect_realize(move |carousel| {
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let carousel = carousel.clone();
        receiver.attach(None, move |_| {
            carousel.scroll_to_full(carousel.nth_page(initial_page).as_ref().unwrap(), 0);
            glib::Continue(false)
        });
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(100));
            sender.send(())
        });
    });
    carousel.connect_page_changed(move |carousel, n| {
        let (lower, upper) = current_range();
        let lower = *lower;
        let upper = *upper;
        let len = upper - lower;
        if n as i32 == 1 {
            let lower = lower - 1;
            carousel.prepend(&item(lower));
            set_current_range((lower, upper));
        } else if n == len as u32 - 2 {
            let upper = upper + 1;
            carousel.append(&item(upper));
            set_current_range((lower, upper));
        }
    });
    carousel
}
