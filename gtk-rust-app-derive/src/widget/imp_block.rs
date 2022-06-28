use quote::{__private::TokenStream, quote};
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Field, ItemImpl, ItemStruct, Path, Token,
};

use super::{
    args::WidgetMacroArgs,
    attributes::*,
    properties::{get_param_specs_from_attrs, get_property_getters, get_property_setters},
    signals::get_signals_definitions,
    store::get_selector_bindings,
};

pub fn get_imp_block(
    input: &ItemStruct,
    args: &WidgetMacroArgs,
    fields: &Punctuated<Field, Comma>,
) -> TokenStream {
    let struct_attrs = &input.attrs;

    let struct_attrs: Vec<&Attribute> = struct_attrs
        .iter()
        .filter(|a| !a.path.is_ident(ATTR_SKIP_AUTO_IMPL))
        .collect();

    let skip_auto_impl = struct_attrs
        .iter()
        .any(|a| a.path.is_ident(ATTR_SKIP_AUTO_IMPL));

    let widget_name = &input.ident;

    let selectors_field = if args.store.is_some() {
        quote! {
            pub(crate) selectors: std::cell::Cell<Vec<crate::gtk_rust_app::gstore::SelectorId>>,
        }
    } else {
        quote! {}
    };

    let struct_fields = get_final_struct_fields(fields);

    let parent = &args.extends;

    let impl_item = if skip_auto_impl {
        None
    } else {
        get_impl_item(widget_name, &args.extends)
    };

    let selector_bindings = if args.store.is_some() {
        get_selector_bindings(fields)
    } else {
        quote!()
    };

    let param_specs = get_param_specs_from_attrs(fields);
    let property_setters = get_property_setters(fields);
    let property_getters = get_property_getters(fields);

    let signals = get_signals_definitions(fields);

    quote!(
        mod imp {
            use super::*;
            use glib::ToValue;
            use gtk::glib;
            use gtk::subclass::prelude::*;
            use libadwaita::subclass::prelude::*;
            use glib::ObjectExt;
            use glib::Cast;
            use glib::subclass::InitializingObject;
            use gtk::subclass::widget::CompositeTemplateCallbacks;

            #[derive(gtk::CompositeTemplate, Default)]
            #(#struct_attrs)*
            pub struct #widget_name {
                pub(crate) signal_handlers: std::cell::Cell<Vec<glib::SignalHandlerId>>,

                #selectors_field

                #struct_fields
            }

            #[glib::object_subclass]
            impl ObjectSubclass for #widget_name {
                const NAME: &'static str = stringify!(#widget_name);
                type Type = super::#widget_name;
                type ParentType = #parent;

                fn class_init(klass: &mut Self::Class) {
                    Self::bind_template(klass);
                }

                fn instance_init(obj: &InitializingObject<Self>) {
                    obj.init_template();
                }
            }

            #impl_item

            impl ObjectImpl for #widget_name {

                fn constructed(&self, obj: &Self::Type) {
                    self.parent_constructed(obj);
                    Self::Type::_constructed(obj);
                }

                fn dispose(&self, obj: &Self::Type) {
                    Self::Type::_dispose(obj);
                }

                fn properties() -> &'static [glib::ParamSpec] {
                    static PROPERTIES: glib::once_cell::sync::Lazy<Vec<glib::ParamSpec>> =
                        glib::once_cell::sync::Lazy::new(|| {
                            vec![
                                #param_specs
                            ]
                        });
                    PROPERTIES.as_ref()
                }

                fn set_property(
                    &self,
                    _obj: &Self::Type,
                    _id: usize,
                    value: &glib::Value,
                    pspec: &glib::ParamSpec,
                ) {
                    match pspec.name().replace("-", "_").as_str() {
                        #property_setters
                        _ => {
                            unimplemented!("prop delegation not implemented")
                        },
                    }
                }

                fn property(
                    &self,
                    _obj: &Self::Type,
                    _id: usize,
                    pspec: &glib::ParamSpec,
                ) -> glib::Value {
                    match pspec.name().replace("-", "_").as_str() {
                        #property_getters
                        _ => {
                            unimplemented!("prop delegation not implemented")
                        },
                    }
                }

                fn signals() -> &'static [glib::subclass::signal::Signal] {
                    use glib::StaticType;
                    static SIGNALS: glib::once_cell::sync::Lazy<Vec<glib::subclass::signal::Signal>> = glib::once_cell::sync::Lazy::new(|| {
                        vec![
                            #signals
                        ]
                    });
                    SIGNALS.as_ref()
                }

            }

            impl #widget_name {
                pub fn _realized(&self, obj: &<Self as ObjectSubclass>::Type) {
                    #selector_bindings
                }
            }

            impl WidgetImpl for #widget_name {}
        }
    )
}

fn get_final_struct_fields(fields: &Punctuated<Field, Comma>) -> Punctuated<Field, Token![,]> {
    let mut filtered_fields = Punctuated::<Field, Token![,]>::new();
    'outer: for field in fields {
        for attr in &field.attrs {
            if attr.path.is_ident(ATTR_SIGNAL) {
                continue 'outer;
            }
            if attr.path.is_ident(ATTR_SIGNAL_HANDLER) {
                continue 'outer;
            }
            if attr.path.is_ident(ATTR_SELECTOR) {
                continue 'outer;
            }
            if attr.path.is_ident(ATTR_DISPOSE) {
                continue 'outer;
            }
            if attr.path.is_ident(ATTR_SIGNAL_RETURNING) {
                continue 'outer;
            }
            if attr.path.get_ident().is_some()
                && attr
                    .path
                    .get_ident()
                    .unwrap()
                    .to_string()
                    .starts_with("property")
            {
                let mut field = field.clone();
                field.attrs = field
                    .attrs
                    .into_iter()
                    .filter(|a| {
                        if let Some(ident) = a.path.get_ident() {
                            !ident.to_string().starts_with("property")
                        } else {
                            false
                        }
                    })
                    .collect();
                filtered_fields.push(field);
                continue 'outer;
            }
        }
        filtered_fields.push(field.clone())
    }
    filtered_fields
}

fn get_impl_item(widget_name: &syn::Ident, parent: &Path) -> Option<ItemImpl> {
    if let Some(ident) = parent.get_ident() {
        let impl_name = quote::format_ident!("{}Impl", ident);
        return syn::parse(proc_macro::TokenStream::from(quote!(
            impl #impl_name for #widget_name {}
        )))
        .ok();
    }
    if let Some(s) = parent.segments.last() {
        let ident = &s.ident;
        let impl_name = quote::format_ident!("{}Impl", ident);
        return syn::parse(proc_macro::TokenStream::from(quote!(
            impl #impl_name for #widget_name {}
        )))
        .ok();
    }

    None
}
