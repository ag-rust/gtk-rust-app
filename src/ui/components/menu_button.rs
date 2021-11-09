use gtk::prelude::*;

use crate::interface;

pub fn menu_button() -> gtk::Widget {
    interface!{r#"
        <menu id="main-menu">
            <section>
                <item>
                    <attribute name="label" translatable="yes">Quit</attribute>
                    <attribute name="action">app.quit</attribute>
                </item>
                <item>
                    <attribute name="label" translatable="yes">About</attribute>
                    <attribute name="action">app.show-about</attribute>
                </item>
            </section>
        </menu>
        <object class="GtkMenuButton" id="menu_button">
            <property name="icon-name">open-menu-symbolic</property>
            <property name="menu-model">main-menu</property>
        </object>
    "#
        menu_button: gtk::MenuButton
    };

    menu_button.upcast()
}
