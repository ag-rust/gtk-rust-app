// SPDX-License-Identifier: GPL-3.0-or-later

extern crate proc_macro;

use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Arm, Expr, ExprReference, Field, Fields, FnArg, Ident, Pat, Path, Result, Token, Type,
};

// gobjectify
// -----------------------------------------------------------------------------------------------------------------
struct Args {
    fields: HashSet<String>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            fields: vars.into_iter().map(|i| i.to_string()).collect(),
        })
    }
}

#[proc_macro_attribute]
pub fn gobjectify(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);
    let input = parse_macro_input!(input as syn::ItemStruct);

    let name = &input.ident;

    let gobject_name = quote::format_ident!("{}GObject", name);
    let gobject_mod_name = quote::format_ident!("gobjectify_{}", name);
    let fields = get_gobject_fields(&input, &args);
    let celled_fields = get_gobject_cell_fields(&fields, Some(&args));
    let field_refs = get_field_refs(&fields, Some(&args));
    let fn_arguments = get_constructor_arguments(&fields, Some(&args));
    let gobject_arguments = get_gobject_constructor_arguments(&fields, Some(&args));
    let param_specs = get_param_specs(&fields, Some(&args));
    let property_getters = get_property_getters(&fields, Some(&args));
    let property_setters = get_property_setters(&fields, Some(&args));

    let gen = quote! {
        #input

        #[allow(non_snake_case)]
        pub mod #gobject_mod_name {
            mod imp {
                use glib::ToValue;
                use gtk::glib;
                use gtk::subclass::prelude::*;

                #[derive(Default)]
                pub struct #gobject_name {
                    #celled_fields
                }

                #[glib::object_subclass]
                impl ObjectSubclass for #gobject_name {
                    const NAME: &'static str = stringify!(#gobject_name);
                    type Type = super::#gobject_name;
                    type ParentType = glib::Object;
                }

                impl ObjectImpl for #gobject_name {
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
                        match pspec.name() {
                            #property_setters
                            _ => unimplemented!(),
                        }
                    }

                    fn property(
                        &self,
                        _obj: &Self::Type,
                        _id: usize,
                        pspec: &glib::ParamSpec,
                    ) -> glib::Value {
                        match pspec.name() {
                            #property_getters
                            _ => unimplemented!(),
                        }
                    }
                }
            }

            use glib::Object;
            use gtk::glib;

            glib::wrapper! {
                pub struct #gobject_name(ObjectSubclass<imp::#gobject_name>) @implements gtk::Accessible;
            }

            impl #gobject_name {
                pub fn new(
                    #fn_arguments
                ) -> Self {
                    Object::new(&[
                        #gobject_arguments
                    ]).expect(&format!("Failed to create {}.", stringify!(#gobject_name)))
                }
            }
        }

        impl #name {
            pub fn to_gobject(&self) -> #gobject_mod_name::#gobject_name {
                #gobject_mod_name::#gobject_name::new(
                    #field_refs
                )
            }

            pub fn gobject_type() -> glib::types::Type {
                use glib::StaticType;
                #gobject_mod_name::#gobject_name::static_type()
            }

            pub fn property_expression(field: &str) -> gtk::Expression {
                let ex = gtk::PropertyExpression::new::<gtk::Expression>(
                    Self::gobject_type(),
                    None,
                    field,
                );
                ex.upcast()
            }
        }
    };

    TokenStream::from(gen)
}

fn get_property_setters(
    fields: &Punctuated<Field, Comma>,
    args: Option<&Args>,
) -> Punctuated<Arm, Token![,]> {
    let mut setters = Punctuated::new();
    for field in fields {
        let ident = field.ident.as_ref().unwrap().clone();
        if args.is_none() || args.unwrap().fields.contains(&ident.to_string()) {
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
    }
    setters
}

fn get_property_getters(
    fields: &Punctuated<Field, Comma>,
    args: Option<&Args>,
) -> Punctuated<Arm, Token![,]> {
    let mut getters = Punctuated::new();
    for field in fields {
        let ident = field.ident.as_ref().unwrap().clone();
        if args.is_none() || args.unwrap().fields.contains(&ident.to_string()) {
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
    }
    getters
}

fn get_param_specs(
    fields: &Punctuated<Field, Comma>,
    filter: Option<&Args>,
) -> Punctuated<Expr, Token![,]> {
    let mut param_specs = Punctuated::<Expr, Token![,]>::new();
    for field in fields {
        let ident = field.ident.as_ref().unwrap().clone();
        let ty = &field.ty;
        if filter.is_none() || filter.unwrap().fields.contains(&ident.to_string()) {
            let param_spec = get_param_spec_for_ty(ident, ty);
            param_specs.push(syn::parse(param_spec).unwrap());
        }
    }
    param_specs
}

fn get_param_spec_for_ty(field_ident: Ident, ty: &Type) -> TokenStream {
    let param_spec = match ty {
        Type::Path(p) => Ok(get_param_spec_for_ident(field_ident, &p.path)),
        Type::Array(_) => Err("Array"),
        Type::BareFn(_) => Err("BareFn"),
        Type::Group(_) => Err("Group"),
        Type::ImplTrait(_) => Err("ImplTrait"),
        Type::Infer(_) => Err("Infer"),
        Type::Macro(_) => Err("Macro"),
        Type::Never(_) => Err("Never"),
        Type::Paren(_) => Err("Paren"),
        Type::Ptr(_) => Err("Ptr"),
        Type::Reference(_) => Err("Reference"),
        Type::Slice(_) => Err("Slice"),
        Type::TraitObject(_) => Err("TraitObject"),
        Type::Tuple(_) => Err("Tuple"),
        Type::Verbatim(_) => Err("Verbatim"),
        Type::__TestExhaustive(_) => Err("__TestExhaustive"),
    };
    match param_spec {
        Ok(ps) => ps,
        Err(e) => {
            unimplemented!("Type {:?} is not implemented in gobjectify", e);
        },
    }
}

fn get_param_spec_for_ident(field_ident: Ident, type_path: &Path) -> TokenStream {
    if let Some(ident) = type_path.get_ident() {
        match ident.to_string().as_str() {
            "String" => {
                TokenStream::from(quote!(
                    //
                    glib::ParamSpec::new_string(
                        &stringify!(#field_ident).replace("_", "-"),
                        &stringify!(#field_ident).replace("_", "-"),
                        &stringify!(#field_ident).replace("_", "-"),
                        None,
                        glib::ParamFlags::READWRITE,
                    )
                ))
            }
            "bool" => {
                TokenStream::from(quote!(
                    //
                    glib::ParamSpec::new_boolean(
                        &stringify!(#field_ident).replace("_", "-"),
                        &stringify!(#field_ident).replace("_", "-"),
                        &stringify!(#field_ident).replace("_", "-"),
                        false,
                        glib::ParamFlags::READWRITE,
                    )
                ))
            }
            _ => {
                unimplemented!(
                    "Type with identifier {:?} is not implemented in gobjectify macro",
                    ident
                );
            }
        }
    } else {
        TokenStream::from(quote!(
            //
            glib::ParamSpec::new_string(
                stringify!(#field_ident),
                stringify!(#field_ident),
                stringify!(#field_ident),
                None,
                glib::ParamFlags::READWRITE,
            )
        ))
    }
}

fn _get_param_spec_for_ident(field_ident: Ident, type_ident: &Ident) -> TokenStream {
    match type_ident.to_string().as_str() {
        "String" => {
            TokenStream::from(quote!(
                //
                glib::ParamSpec::new_string(
                    stringify!(#field_ident),
                    stringify!(#field_ident),
                    stringify!(#field_ident),
                    None,
                    glib::ParamFlags::READWRITE,
                )
            ))
        }
        _ => {
            // warn!("Ident type is implemented but not {:?} in gobjectify macro", type_ident);
            TokenStream::from(quote!(
                //
                glib::ParamSpec::new_string(
                    stringify!(#field_ident),
                    stringify!(#field_ident),
                    stringify!(#field_ident),
                    None,
                    glib::ParamFlags::READWRITE,
                )
            ))
        }
    }
}

fn get_gobject_constructor_arguments(
    fields: &Punctuated<Field, Comma>,
    args: Option<&Args>,
) -> Punctuated<Pat, Token![,]> {
    let mut arguments = Punctuated::<Pat, Token![,]>::new();
    for field in fields {
        let ident = field.ident.as_ref().unwrap().clone();
        if args.is_none() || args.unwrap().fields.contains(&ident.to_string()) {
            let tuple = TokenStream::from(quote!(
                (stringify!(#ident), &#ident)
            ));
            let tuple = syn::parse::<Pat>(tuple).unwrap();
            arguments.push(tuple);
        }
    }
    arguments
}

fn get_constructor_arguments(
    fields: &Punctuated<Field, Comma>,
    args: Option<&Args>,
) -> Punctuated<FnArg, Token![,]> {
    let mut arguments = Punctuated::<FnArg, Token![,]>::new();
    for field in fields {
        let ident = field.ident.as_ref().unwrap().clone();
        let ty = &field.ty;
        if args.is_none() || args.unwrap().fields.contains(&ident.to_string()) {
            let fn_attribute = TokenStream::from(quote!(
                #ident: &#ty
            ));
            arguments.push(syn::parse::<FnArg>(fn_attribute).unwrap());
        }
    }
    arguments
}

fn get_field_refs(
    fields: &Punctuated<Field, Comma>,
    args: Option<&Args>,
) -> Punctuated<ExprReference, Token![,]> {
    let mut field_refs = Punctuated::<ExprReference, Token![,]>::new();
    for field in fields {
        let ident = field.ident.as_ref().unwrap().clone();
        if args.is_none() || args.unwrap().fields.contains(&ident.to_string()) {
            let field_ref = TokenStream::from(quote!(
                &self.#ident
            ));
            field_refs.push(syn::parse::<ExprReference>(field_ref).unwrap());
        }
    }
    field_refs
}

fn get_gobject_fields(input: &syn::ItemStruct, args: &Args) -> Punctuated<Field, Token![,]> {
    let fields = match &input.fields {
        Fields::Named(fields_named) => {
            let mut filtered_fields = Punctuated::new();
            for field in fields_named.named.iter() {
                let ident = field.ident.as_ref().unwrap().clone();
                if args.fields.contains(&ident.to_string()) {
                    filtered_fields.push(field.clone());
                }
            }
            filtered_fields
        }
        Fields::Unnamed(_) => unimplemented!("Field with unnamed type not implemented"),
        Fields::Unit => unimplemented!("Field with unit type not implemented"),
    };
    fields
}

fn get_gobject_cell_fields(
    fields: &Punctuated<Field, Comma>,
    args: Option<&Args>,
) -> Punctuated<Field, Token![,]> {
    let mut cell_fields = Punctuated::<Field, Token![,]>::new();
    for field in fields {
        let ident = field.ident.as_ref().unwrap().clone();
        if args.is_none() || args.unwrap().fields.contains(&ident.to_string()) {
            let ty = &field.ty;
            let field_ty = syn::parse::<Type>(TokenStream::from(quote!(
                std::cell::Cell<#ty>
            )))
            .unwrap();
            let mut field = field.clone();
            field.ty = field_ty;
            cell_fields.push(field);
        }
    }
    cell_fields
}

// widget
// -----------------------------------------------------------------------------------------------------------------

#[proc_macro_attribute]
pub fn widget(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemFn);

    let function_name = &input.sig.ident;
    let return_type = if let syn::ReturnType::Type(_, return_type) = &input.sig.output {
        if let syn::Type::Path(return_type) = return_type.as_ref() {
            if let Some(return_type) = return_type.path.get_ident() {
                return_type
            } else {
                panic!("Wrong return type for widget function.");
            }
        } else {
            panic!("Wrong return type for widget function.");
        }
    } else {
        panic!("Wrong return type for widget function.");
    };
    let widget_name = quote::format_ident!("{}", return_type);
    let widget_mod_name = quote::format_ident!("widget_{}", function_name);

    let fields = get_gobject_fields_from_function(&input);
    let celled_fields = get_gobject_cell_fields(&fields, None);
    let param_specs = get_param_specs(&fields, None);
    let property_setters = get_property_setters(&fields, None);
    let property_getters = get_property_getters(&fields, None);
    let constructor_arguments = get_constructor_arguments(&fields, None);
    let gobject_construct_arguments = get_gobject_constructor_arguments(&fields, None);

    let gen = quote! {
        #input

        #[allow(non_snake_case)]
        mod #widget_mod_name {
            mod imp {
                use glib::ToValue;
                use gtk::glib;
                use gtk::subclass::prelude::*;
                use glib::ObjectExt;
                use glib::Cast;

                #[derive(Default)]
                pub struct #widget_name {
                    #celled_fields
                }

                #[glib::object_subclass]
                impl ObjectSubclass for #widget_name {
                    const NAME: &'static str = stringify!(#widget_name);
                    type Type = super::#widget_name;
                    type ParentType = gtk::Widget;
                }

                impl ObjectImpl for #widget_name {
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
                        match pspec.name() {
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
                        match pspec.name() {
                            #property_getters
                            _ => {
                                unimplemented!("prop delegation not implemented")
                            },
                        }
                    }
                }

                impl WidgetImpl for #widget_name {}
            }

            use glib::Object;
            use gtk::glib;

            glib::wrapper! {
                pub struct #widget_name(ObjectSubclass<imp::#widget_name>)
                @extends gtk::Widget, gtk::Box,
                @implements gtk::Accessible;
            }

            impl #widget_name {
                pub fn new(
                    #constructor_arguments
                ) -> Self {
                    Object::new(&[
                        #gobject_construct_arguments
                    ]).expect(&format!("Failed to create {}.", stringify!(#widget_name)))
                }
            }
        }

        pub use #widget_mod_name::#widget_name;
    };

    TokenStream::from(gen)
}

fn get_gobject_fields_from_function(input: &syn::ItemFn) -> Punctuated<Field, Token![,]> {
    let mut fields: Punctuated<Field, Comma> = Punctuated::new();
    for arg in &input.sig.inputs {
        match arg {
            FnArg::Receiver(_) => panic!("Self argument not supported for widget macro"),
            FnArg::Typed(arg) => {
                let ident = match arg.pat.as_ref() {
                    Pat::Ident(ident) => &ident.ident,
                    _ => unimplemented!("Function arguments other than simple identifiers are not supported right now"),
                };
                let ty = arg.ty.as_ref();
                let field = syn::parse::<syn::FieldsNamed>(
                    quote!(
                        {
                        pub #ident: #ty
                        }
                    )
                    .into(),
                )
                .expect("Could not generate field names from function arguments");
                let field = field
                    .named
                    .first()
                    .expect("Could get single field from generated fields")
                    .clone();
                fields.push(field);
            }
        };
    }
    fields
}
