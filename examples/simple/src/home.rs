// SPDX-License-Identifier: GPL-3.0-or-later

use gtk::prelude::*;

// Define a UI component with a function
pub fn home() -> gtk::Widget {
    // The interface macro allows to define your UI with the common gtk ui XML structures.
    interface!(r#"
        <object class="GtkBox" id="page">
            <property name="visible">True</property>
            <property name="orientation">vertical</property>
            <property name="spacing">16</property>

            <child>
            <object class="GtkLabel" id="label">
                <property name="label">gettext("Home page")</property>
            </object>
            </child>
            
            <child>
            <object class="GtkButton" id="button">
                <property name="visible">True</property>
                <property name="label">gettext("Press me")</property>
            </object>
            </child>

        </object>
    "#
        // Each widget which has an id can be retrieved here as a variable.
        page: gtk::Box,
        label: gtk::Label,
        button: gtk::Button,
    );

    println!("Do stuff with the widgets: {:?} {:?}", label, button);

    page.upcast()
}
