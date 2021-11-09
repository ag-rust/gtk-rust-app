use crate::interface;
use gtk::prelude::*;

pub fn about_view() -> gtk::Widget {
    interface! {r#"
        <object class="GtkBox" id="view">
            <property name="orientation">vertical</property>
            <property name="margin-top">12</property>
            <property name="margin-bottom">12</property>
            <property name="margin-start">12</property>
            <property name="margin-end">12</property>
            <property name="spacing">6</property>
        </object>
    "#
        view: gtk::Box
    };
    
    view.upcast()
}
