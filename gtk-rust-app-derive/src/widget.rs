// SPDX-License-Identifier: GPL-3.0-or-later

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Field,
    Fields, Ident,
};

mod attributes;
use attributes::*;
mod args;
use args::*;
mod imp_block;
mod store;
use imp_block::*;

use self::signals::{get_signal_connectors, get_signal_emitters, get_signal_handlers};
mod properties;
mod signals;

pub fn widget(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as WidgetMacroArgs);
    let input = parse_macro_input!(input as syn::ItemStruct);

    let widget_name = &input.ident;
    let fields = &input.fields;
    let fields = match fields {
        Fields::Named(fields) => Ok(&fields.named),
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

    let imp_block = get_imp_block(&input, &args, fields);

    let signal_connectors = get_signal_connectors(fields);
    let signal_emitters = get_signal_emitters(fields);
    let signal_handlers = get_signal_handlers(fields);

    let template_child_accessors = get_template_child_accessors(fields);

    let parent = args.extends;
    let implements = args.implements;

    let gen = quote! {

        mod imp {
            use super::*;
            use glib::ToValue;
            use gtk::glib;
            use gtk::subclass::prelude::*;
            #[cfg(feature = "libadwaita")]
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
    let dispose = if let Some(dispose_function) = get_dispose_function(fields) {
        quote! {
            self.#dispose_function()
        }
    } else {
        quote! {}
    };

    let store_cleanup = if let Some(store) = &args.store {
        quote! {
            for s in self.imp().selectors.take() {
                #store().deselect(s)
            }
        }
    } else {
        quote! {}
    };

    // final code gen

    let gen = quote! {

        #imp_block

        use glib::Object;
        use gtk::glib;
        use glib::closure_local;

        glib::wrapper! {
            pub struct #widget_name(ObjectSubclass<imp::#widget_name>)
            @extends #parent,
            @implements #implements;
        }

        impl #widget_name {

            fn _constructed(&self) {
                #(#signal_handlers)*
                self.constructed();
                self.connect_realize(|_self| {
                    gdk4::subclass::prelude::ObjectSubclassIsExt::imp(_self)._realized(_self);
                });
            }

            #(#template_child_accessors)*

            #(#signal_connectors)*

            #(#signal_emitters)*
        }

        impl #widget_name {
            fn auto_cleanup_signal_handler(&self, signal_handler_id: glib::SignalHandlerId) {
                use gdk4::subclass::prelude::ObjectSubclassIsExt;

                let mut sh = self.imp().signal_handlers.take();
                sh.push(signal_handler_id);
                self.imp().signal_handlers.set(sh);
            }

            fn _dispose(&self) {
                use gdk4::subclass::prelude::ObjectSubclassIsExt;

                for sh in self.imp().signal_handlers.take() {
                    self.disconnect(sh);
                }

                #store_cleanup

                #dispose
            }
        }

    };

    if std::env::var("GRA_PRINT_GEN").is_ok() {
        println!();
        println!();
        println!("//###########################################");
        println!("//### {} ", widget_name);
        println!("//###########################################");
        println!("{}", gen);
        println!("//###########################################");
    }

    TokenStream::from(gen)
}

fn get_dispose_function(fields: &Punctuated<Field, Comma>) -> Option<Ident> {
    fields
        .iter()
        .find(|f| get_attr(f, ATTR_DISPOSE).is_some())
        .and_then(|f| f.ident.clone())
}

fn get_template_child_accessors(fields: &Punctuated<Field, Comma>) -> Vec<syn::ImplItemMethod> {
    let mut methods = Vec::new();
    for field in fields {
        if let Some(ident) = field.ident.as_ref() {
            if get_template_child_attr(field).is_some() {
                let ty = match &field.ty {
                    syn::Type::Path(p) => {
                        let arguments = &p.path.segments.first().unwrap().arguments;
                        match arguments {
                            syn::PathArguments::AngleBracketed(arg) => {
                                match arg.args.first().unwrap() {
                                    syn::GenericArgument::Type(t) => t,
                                    _ => unreachable!(
                                        "Template child fields must have one type argument"
                                    ),
                                }
                            }
                            _ => {
                                unreachable!("Template child fields must have a generic argument.")
                            }
                        }
                    }
                    _ => unreachable!("Template child fields must have a Path type."),
                };
                methods.push(
                    syn::parse(TokenStream::from(quote! {
                        fn #ident(&self) -> &#ty {
                            &self.imp().#ident
                        }
                    }))
                    .unwrap(),
                );
            }
        }
    }
    methods
}

fn get_template_child_attr(field: &Field) -> Option<&Attribute> {
    for attr in &field.attrs {
        if attr.path.is_ident(ATTR_TEMPLATE_CHILD) {
            return Some(attr);
        }
    }
    None
}
