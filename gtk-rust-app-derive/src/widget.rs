// SPDX-License-Identifier: GPL-3.0-or-later

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, Arm, Attribute,
    Expr, Field, Fields, ItemFn, ItemImpl, Path, Token,
};

const ATTR_SKIP_AUTO_IMPL: &str = "skip_auto_impl";

const ATTR_SIGNAL: &str = "signal";
const ATTR_SIGNAL_RETURNING: &str = "signal_returning";
const ATTR_CALLBACK: &str = "callback";

const ATTR_PROPERTY: &str = "property";
const ATTR_PROPERTY_STRING: &str = "property_string";
const ATTR_PROPERTY_BOOL: &str = "property_bool";
const ATTR_PROPERTY_U64: &str = "property_u64";
const ATTR_PROPERTY_I64: &str = "property_i64";
const ATTR_PROPERTY_F64: &str = "property_f64";

pub fn widget(args: TokenStream, input: TokenStream) -> TokenStream {
    let parent = parse_macro_input!(args as Path);
    let input = parse_macro_input!(input as syn::ItemStruct);

    let struct_attrs = input.attrs;

    let skip_auto_impl = struct_attrs
        .iter()
        .any(|a| a.path.is_ident(ATTR_SKIP_AUTO_IMPL));

    let struct_attrs: Vec<Attribute> = struct_attrs
        .into_iter()
        .filter(|a| !a.path.is_ident(ATTR_SKIP_AUTO_IMPL))
        .collect();

    let widget_name = input.ident;
    let fields = input.fields;
    let fields = match fields {
        Fields::Named(fields) => Ok(fields.named),
        Fields::Unnamed(_) => Err(syn::Error::new(
            fields.span(),
            "Widget structs support named fields only",
        )),
        Fields::Unit => Err(syn::Error::new(
            fields.span(),
            "Widget structs support named fields only",
        )),
    }
    .unwrap();

    let struct_fields = get_final_struct_fields(&fields);

    let property_fields = get_fields_with_property_attr(&fields);
    let param_specs = get_param_specs_from_attrs(&property_fields);
    let property_setters = get_property_setters(&property_fields);
    let property_getters = get_property_getters(&property_fields);

    let signals = get_signals_definitions(&fields);
    let signal_connectors = get_signal_connectors(&fields);
    let signal_emitters = get_signal_emitters(&fields);

    let impl_item;
    if skip_auto_impl {
        impl_item = None;
    } else {
        impl_item = get_impl_item(&widget_name, &parent);
    }

    let template_callbacks = get_template_callbacks(&fields);

    let gen = quote! {

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
                #struct_fields
            }

            #[glib::object_subclass]
            impl ObjectSubclass for #widget_name {
                const NAME: &'static str = stringify!(#widget_name);
                type Type = super::#widget_name;
                type ParentType = #parent;

                fn class_init(klass: &mut Self::Class) {
                    Self::bind_template(klass);
                    // Self::bind_template_callbacks(klass);
                    Self::Type::bind_template_callbacks(klass);
                }

                fn instance_init(obj: &InitializingObject<Self>) {
                    obj.init_template();
                }
            }

            #impl_item

            impl ObjectImpl for #widget_name {

                fn constructed(&self, obj: &Self::Type) {
                    self.parent_constructed(obj);
                    Self::Type::constructed(obj);
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

            impl WidgetImpl for #widget_name {}
        }

        use glib::Object;
        use gtk::glib;
        use glib::closure_local;

        glib::wrapper! {
            pub struct #widget_name(ObjectSubclass<imp::#widget_name>)
            @extends gtk::Box,
            @implements gtk::Widget, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
        }

        impl #widget_name {
            #(#signal_connectors)*

            #(#signal_emitters)*
        }

        #[gtk::template_callbacks]
        impl #widget_name {
            #(
            #template_callbacks
            )*
        }

    };

    if std::env::var("GRA_PRINT_GEN").is_ok() {
        println!();
        println!();
        println!("###########################################");
        println!("### {} ", widget_name);
        println!("###########################################");
        println!("{}", gen);
        println!("###########################################");
    }

    TokenStream::from(gen)
}

fn get_property_setters(fields: &Punctuated<Field, Comma>) -> Punctuated<Arm, Token![,]> {
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

fn get_property_getters(fields: &Punctuated<Field, Comma>) -> Punctuated<Arm, Token![,]> {
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

fn get_final_struct_fields(fields: &Punctuated<Field, Comma>) -> Punctuated<Field, Token![,]> {
    let mut filtered_fields = Punctuated::<Field, Token![,]>::new();
    'outer: for field in fields {
        for attr in &field.attrs {
            if attr.path.is_ident(ATTR_SIGNAL) {
                continue 'outer;
            }
            if attr.path.is_ident(ATTR_SIGNAL_RETURNING) {
                continue 'outer;
            }
            if attr.path.is_ident(ATTR_CALLBACK) {
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

fn get_param_specs_from_attrs(fields: &Punctuated<Field, Comma>) -> Punctuated<Expr, Token![,]> {
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

fn get_signals_definitions(fields: &Punctuated<Field, Comma>) -> Punctuated<Expr, Token![,]> {
    let mut signal_definitions: Punctuated<Expr, Comma> = Punctuated::new();
    for field in fields {
        if let Some(attr) = get_signal_attr(field) {
            let sd: Option<Expr> = syn::parse2(attr.tokens.clone()).ok();
            if let Some(sd) = sd {
                signal_definitions.push(sd);
            } else {
                let name = field.ident.as_ref().unwrap();
                let sd = syn::parse::<syn::Expr>(
                    quote!(glib::subclass::signal::Signal::builder(
                        &stringify!(#name).replace("_", "-"),
                        &[],
                        <()>::static_type().into(),
                    )
                    .build())
                    .into(),
                )
                .expect("Could not generate signals from macro argument");
                signal_definitions.push(sd);
            }
        } else if let Some(attr) = get_signal_ret_attr(field) {
            let ty = get_signal_return_type(attr);

            let name = field.ident.as_ref().unwrap();
            let sd = syn::parse::<syn::Expr>(
                quote!(glib::subclass::signal::Signal::builder(
                    &stringify!(#name).replace("_", "-"),
                    &[#ty::static_type().into()],
                    <()>::static_type().into(),
                )
                .build())
                .into(),
            )
            .expect("Could not generate signals from macro argument");
            signal_definitions.push(sd);
        }
    }
    signal_definitions
}

fn get_signal_return_type(attr: &Attribute) -> syn::Ident {
    let s = attr.tokens.to_string();
    let s = &s[1..s.len() - 1];
    let ty = quote::format_ident!("{}", s);
    ty
}

fn get_signal_attr(field: &Field) -> Option<&Attribute> {
    for attr in &field.attrs {
        if attr.path.is_ident(ATTR_SIGNAL) {
            return Some(attr);
        }
    }
    None
}

fn get_signal_ret_attr(field: &Field) -> Option<&Attribute> {
    for attr in &field.attrs {
        if attr.path.is_ident(ATTR_SIGNAL_RETURNING) {
            return Some(attr);
        }
    }
    None
}

fn get_signal_connectors(fields: &Punctuated<Field, Comma>) -> Vec<ItemFn> {
    let mut signal_connectors: Vec<ItemFn> = Vec::new();

    for field in fields {
        if let Some(attr) = get_signal_attr(field) {
            let sd: Option<Expr> = syn::parse2(attr.tokens.clone()).ok();
            if sd.is_some() {
                // noop
            } else {
                let name = field.ident.as_ref().unwrap();
                let connector_ident = quote::format_ident!("_connect_{}", name);
                let connector = syn::parse::<syn::ItemFn>(
                    quote!(
                        //
                        fn #connector_ident(&self, f: impl Fn(&Self) + 'static) {
                            self.connect_closure(
                                stringify!(#name),
                                false,
                                closure_local!(move |s: Self| {
                                    f(&s);
                                }),
                            );
                        }
                        //
                    )
                    .into(),
                )
                .expect("Could not generate signals from macro argument");
                signal_connectors.push(connector);
            }
        } else if let Some(attr) = get_signal_ret_attr(field) {
            let ty = get_signal_return_type(attr);

            let name = field.ident.as_ref().unwrap();
            let connector_ident = quote::format_ident!("_connect_{}", name);
            let connector = syn::parse::<syn::ItemFn>(
                quote!(
                    //
                    fn #connector_ident(&self, f: impl Fn(&Self, #ty) + 'static) {
                        self.connect_closure(
                            stringify!(#name),
                            false,
                            closure_local!(move |s: Self, v: #ty| {
                                f(&s, v);
                            }),
                        );
                    }
                    //
                )
                .into(),
            )
            .expect("Could not generate signals from macro argument");
            signal_connectors.push(connector);
        }
    }
    signal_connectors
}

fn get_signal_emitters(fields: &Punctuated<Field, Comma>) -> Vec<ItemFn> {
    let mut signal_emitters: Vec<ItemFn> = Vec::new();

    for field in fields {
        if let Some(attr) = get_signal_attr(field) {
            let sd: Option<Expr> = syn::parse2(attr.tokens.clone()).ok();
            if sd.is_some() {
                // noop
            } else {
                let name = field.ident.as_ref().unwrap();
                let emitter_name = quote::format_ident!("emit_{}", name);
                let emitter = syn::parse::<syn::ItemFn>(
                    quote!(
                        //
                        fn #emitter_name(&self) {
                            self.emit_by_name::<()>(&stringify!(#name).replace("_", "-"), &[]);
                        }
                        //
                    )
                    .into(),
                )
                .expect("Could not generate signal emitter from macro argument");
                signal_emitters.push(emitter);
            }
        } else if let Some(attr) = get_signal_ret_attr(field) {
            let ty = get_signal_return_type(attr);
            let name = field.ident.as_ref().unwrap();
            let emitter_name = quote::format_ident!("emit_{}", name);
            let emitter = syn::parse::<syn::ItemFn>(
                quote!(
                    //
                    fn #emitter_name(&self, v: #ty) {
                        self.emit_by_name::<()>(&stringify!(#name).replace("_", "-"), &[&v.to_value()]);
                    }
                    //
                )
                .into(),
            )
            .expect("Could not generate signal emitter from macro argument");
            signal_emitters.push(emitter);
        }
    }
    signal_emitters
}

fn get_impl_item(widget_name: &syn::Ident, parent: &Path) -> Option<ItemImpl> {
    if let Some(ident) = parent.get_ident() {
        let impl_name = quote::format_ident!("{}Impl", ident);
        return syn::parse(TokenStream::from(quote!(
            impl #impl_name for #widget_name {}
        )))
        .ok();
    }
    if let Some(s) = parent.segments.last() {
        let ident = &s.ident;
        let impl_name = quote::format_ident!("{}Impl", ident);
        return syn::parse(TokenStream::from(quote!(
            impl #impl_name for #widget_name {}
        )))
        .ok();
    }

    None
}

fn is_callback_field(field: &Field) -> bool {
    for attr in &field.attrs {
        if attr.path.is_ident(ATTR_CALLBACK) {
            return true;
        }
    }
    false
}

fn get_template_callbacks(fields: &Punctuated<Field, Comma>) -> Vec<ItemFn> {
    let mut callbacks: Vec<ItemFn> = Vec::new();

    for field in fields {
        if is_callback_field(field) {
            let name = field.ident.as_ref().unwrap();

            let callback_name_str = name.to_string();
            let callback_name = quote::format_ident!("{}_imp", name);

            let callback = syn::parse::<syn::ItemFn>(
                quote!(
                    //
                    #[template_callback(name = #callback_name_str)]
                    fn #callback_name(&self, widget: gtk::Widget) {
                        self.#name(
                            widget.downcast()
                            .expect(&format!("Callback '{}' argument can not be cast to the given type.", stringify!(#name)))
                        );
                    }
                    //
                )
                .into(),
            )
            .unwrap_or_else(|_| panic!("Could not generate signals from macro argument `{}`",
                field.to_token_stream()));
            callbacks.push(callback);
        }
    }
    callbacks
}
