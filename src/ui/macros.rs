#[macro_export]
macro_rules! interface {
    (
        $ui:literal
        $($ref:ident: $ref_type:ty),*$(,)?
    ) => {
        let builder = gtk::Builder::from_string(&format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <interface>
            {}
            </interface>
            "#,
            $ui
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
