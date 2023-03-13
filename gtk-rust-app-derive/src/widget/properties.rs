use super::attributes::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Arm, Attribute, Expr, Field, Token};

fn get_property_attr(field: &Field) -> Option<&Attribute> {
    for attr in &field.attrs {
        if let syn::AttrStyle::Outer = attr.style {
            if let Some(ident) = attr.path.get_ident() {
                if ident.to_string().starts_with(ATTR_PROPERTY) {
                    return Some(attr);
                }
            }
        }
    }
    None
}

fn get_fields_with_property_attr(
    fields: &Punctuated<Field, Comma>,
) -> Punctuated<Field, Token![,]> {
    let mut filtered_fields = Punctuated::<Field, Token![,]>::new();
    for f in fields {
        if get_property_attr(f).is_some() {
            filtered_fields.push(f.clone())
        }
    }
    filtered_fields
}

pub fn get_param_specs_from_attrs(
    fields: &Punctuated<Field, Comma>,
) -> Punctuated<Expr, Token![,]> {
    let fields = get_fields_with_property_attr(fields);

    let mut param_specs = Punctuated::<Expr, Token![,]>::new();
    for field in fields {
        let field_ident = field.ident.as_ref().unwrap();
        for attr in &field.attrs {
            if let syn::AttrStyle::Outer = attr.style {
                if let Some(ident) = attr.path.get_ident() {
                    if !ident.to_string().starts_with(ATTR_PROPERTY) {
                        continue;
                    }

                    if *ident == ATTR_PROPERTY_STRING {
                        param_specs.push(
                            syn::parse(TokenStream::from(quote!(
                                //
                                glib::ParamSpecString::new(
                                    &stringify!(#field_ident).replace("_", "-"),
                                    "",
                                    "",
                                    None,
                                    glib::ParamFlags::READWRITE,
                                )
                            )))
                            .unwrap(),
                        );
                        continue;
                    }

                    if *ident == ATTR_PROPERTY_BOOL {
                        param_specs.push(
                            syn::parse(TokenStream::from(quote!(
                                //
                                glib::ParamSpecBoolean::new(
                                    &stringify!(#field_ident).replace("_", "-"),
                                    "",
                                    "",
                                    false,
                                    glib::ParamFlags::READWRITE,
                                )
                            )))
                            .unwrap(),
                        );
                        continue;
                    }

                    if *ident == ATTR_PROPERTY_I64 {
                        param_specs.push(
                            syn::parse(TokenStream::from(quote!(
                                //
                                glib::ParamSpecInt64::new(
                                    &stringify!(#field_ident).replace("_", "-"),
                                    "",
                                    "",
                                    i64::MIN,
                                    i64::MAX,
                                    0,
                                    glib::ParamFlags::READWRITE,
                                )
                            )))
                            .unwrap(),
                        );
                        continue;
                    }

                    if *ident == ATTR_PROPERTY_U64 {
                        param_specs.push(
                            syn::parse(TokenStream::from(quote!(
                                //
                                glib::ParamSpecUInt64::new(
                                    &stringify!(#field_ident).replace("_", "-"),
                                    "",
                                    "",
                                    u64::MIN,
                                    u64::MAX,
                                    0,
                                    glib::ParamFlags::READWRITE,
                                )
                            )))
                            .unwrap(),
                        );
                        continue;
                    }

                    if *ident == ATTR_PROPERTY_F64 {
                        param_specs.push(
                            syn::parse(TokenStream::from(quote!(
                                //
                                glib::ParamSpecDouble::new(
                                    &stringify!(#field_ident).replace("_", "-"),
                                    "",
                                    "",
                                    f64::MIN,
                                    f64::MAX,
                                    0.,
                                    glib::ParamFlags::READWRITE,
                                )
                            )))
                            .unwrap(),
                        );
                        continue;
                    }

                    let e: Expr = syn::parse2(attr.tokens.clone()).unwrap();
                    param_specs.push(e);
                }
            }
        }
    }
    param_specs
}

pub fn get_property_setters(fields: &Punctuated<Field, Comma>) -> Punctuated<Arm, Token![,]> {
    let fields = get_fields_with_property_attr(fields);

    let mut setters = Punctuated::new();
    for field in fields {
        let ident = field.ident.as_ref().unwrap().clone();

        setters.push(
            syn::parse(TokenStream::from(quote!(

                stringify!(#ident) => {
                    let value = value.get().expect("Argument of wrong type");
                    self.#ident.replace(value);
                }

            )))
            .unwrap(),
        );
    }
    setters
}

pub fn get_property_getters(fields: &Punctuated<Field, Comma>) -> Punctuated<Arm, Token![,]> {
    let fields = get_fields_with_property_attr(fields);

    let mut getters = Punctuated::new();
    for field in fields {
        let ident = field.ident.as_ref().unwrap().clone();

        getters.push(
            syn::parse(TokenStream::from(quote!(

                stringify!(#ident) => {
                    let rust_value = self.#ident.take();
                    let gobject_value = rust_value.to_value();
                    self.#ident.replace(rust_value);
                    gobject_value
                }

            )))
            .unwrap(),
        );
    }
    getters
}
