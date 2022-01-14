// SPDX-License-Identifier: GPL-3.0-or-later

#[macro_export]
macro_rules! __interface {
    (
        $regex:ty,
        $ui:literal
        $($ref:ident: $ref_type:ty),*$(,)?
    ) => {
        let translation_regex = <$regex>::new(r#">gettext\("(?P<t>.*)"\)<"#).unwrap();
        let builder = gtk::Builder::from_string(&format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <interface>
            {}
            </interface>
            "#,
            translation_regex.replace_all($ui, r#" translatable="yes">$t<"#)
        ));
        $(
        let $ref: $ref_type = builder.object(stringify!($ref))
            .expect(&format!(
                "Widget with id {} does not exist",
                stringify!($ref)
            ));
        )*
    }
}

#[macro_export]
macro_rules! interface {
    (
        $($tokens:tt)*
    ) => {
        gtk_rust_app::__interface!(gtk_rust_app::regex::Regex, $($tokens)*)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let re = regex::Regex::new(r#">gettext\("(?P<t>.*)"\)<"#).unwrap();
        let a = re.replace_all(
            r#"
        <interface>
            <object>
                <property name="Test">gettext("Test")</property>
            </object>
            <object>
                <property name="Test2">gettext("Bla")</property>
            </object>
        </interface
        "#,
            r#" translatable="yes">$t<"#,
        );
        assert_eq!(
            a,
            r#"
        <interface>
            <object>
                <property name="Test" translatable="yes">Test</property>
            </object>
            <object>
                <property name="Test2" translatable="yes">Bla</property>
            </object>
        </interface
        "#
        );
    }
}
// #[macro_export]
// macro_rules! action {
//     () => {
//         const DESCRIPTOR: &str = include_str!("Cargo.toml");
//     }
// }
// mod imp {
//     #[derive(Default)]
//     pub struct MyObject;

//     #[glib::object_subclass]
//     impl ObjectSubclass for MyObject {
//         const NAME: &'static str = "MyObject";

//         type Type = super::MyObject;
//         type ParentType = glib::Object;
//     }

//     impl ObjectImpl for MyObject {}
// }

// glib::wrapper! {
//     pub struct MyObject(ObjectSubclass<imp::MyObject>);
// }

// impl MyObject {
//     pub fn new() -> Self {
//         glib::Object::new(&[]).unwrap()
//     }
// }
