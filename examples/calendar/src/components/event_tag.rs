use gtk::prelude::*;

pub fn event_tag(label: &str) -> gtk::Widget {
    interface! {r#"
        <object class="GtkBox" id="event_tag">
            <property name="visible">True</property>
            <property name="can-focus">False</property>
            <property name="orientation">vertical</property>
            <property name="margin-start">2</property>
            <property name="margin-end">2</property>
            <property name="hexpand">true</property>
            <property name="halign">fill</property>
            <style>
                <class name="event-tag"/>
            </style>
            <child>
                <object class="GtkLabel" id="event_tag_label">
                    <property name="visible">True</property>
                    <property name="can-focus">False</property>
                    <property name="halign">center</property>
                    <property name="valign">center</property>
                    <property name="margin-start">4</property>
                    <property name="margin-end">4</property>
                </object>
            </child>
        </object>
    "#
        event_tag: gtk::Box,
        event_tag_label: gtk::Label,
    }
    event_tag_label.set_label(label);
    event_tag.upcast()
}
