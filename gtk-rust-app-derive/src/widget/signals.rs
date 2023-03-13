use super::attributes::*;
use quote::quote;
use syn::{
    parenthesized,
    parse::Parse,
    punctuated::Punctuated,
    token::{Comma, Paren},
    Attribute, Block, Expr, Field, Ident, ItemFn, Token,
};

pub fn get_signals_definitions(fields: &Punctuated<Field, Comma>) -> Punctuated<Expr, Token![,]> {
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

pub fn get_signal_attr(field: &Field) -> Option<&Attribute> {
    for attr in &field.attrs {
        if attr.path.is_ident(ATTR_SIGNAL) {
            return Some(attr);
        }
    }
    None
}

pub fn get_signal_handler_attr(field: &Field) -> Option<&Attribute> {
    for attr in &field.attrs {
        if attr.path.is_ident(ATTR_SIGNAL_HANDLER) {
            return Some(attr);
        }
    }
    None
}

pub fn get_signal_ret_attr(field: &Field) -> Option<&Attribute> {
    for attr in &field.attrs {
        if attr.path.is_ident(ATTR_SIGNAL_RETURNING) {
            return Some(attr);
        }
    }
    None
}

enum SignalHandlerAttrArguments {
    WithSelf {
        _parens: Paren,
        _self_token: Token![self],
        signal: Ident,
    },
    WithIdent {
        _parens: Paren,
        child: Ident,
        signal: Ident,
    },
}

impl Parse for SignalHandlerAttrArguments {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let _parens = parenthesized!(content in input);
        if content.lookahead1().peek(Token![self]) {
            Ok(SignalHandlerAttrArguments::WithSelf {
                _parens,
                _self_token: content.parse()?,
                signal: content.parse()?,
            })
        } else {
            Ok(SignalHandlerAttrArguments::WithIdent {
                _parens,
                child: content.parse()?,
                signal: content.parse()?,
            })
        }
    }
}

/// parse
/// ```
/// #[signal_handler(button: clicked)]
/// on_button_click: ()
/// ```
/// to this
/// ```
/// imp A {
///     constructed(&self) {
///         let _self = self;
///         self.button().connect_closure(
///             "clicked",
///             false,
///             closure_local!(move |b: Button| {
///                 _self.on_button_click(b)
///             }
///         ))
///     }
/// }
/// ```
///
pub fn get_signal_handlers(fields: &Punctuated<Field, Comma>) -> Vec<Block> {
    let mut blocks = Vec::new();
    for field in fields {
        if let Some(attr) = get_signal_handler_attr(field) {
            let attr_tokens = attr.tokens.clone();
            let args = syn::parse2::<SignalHandlerAttrArguments>(attr_tokens).unwrap();
            let handler_name = field.ident.as_ref().unwrap();
            let block = match args {
                SignalHandlerAttrArguments::WithSelf {
                    _parens,
                    _self_token,
                    signal,
                } => {
                    syn::parse::<syn::Block>(
                        quote!(
                            {
                                let _self = self;
                                let signal_handler_id = self.connect_closure(
                                    stringify!(#signal),
                                    false,
                                    closure_local!(@watch _self => move |b: gtk::Widget| { _self.#handler_name(b.downcast().unwrap()) }),
                                );
                                self.auto_cleanup_signal_handler(signal_handler_id);
                            }
                        )
                        .into(),
                    )
                    .unwrap()
                },
                SignalHandlerAttrArguments::WithIdent {
                    _parens,
                    child,
                    signal,
                } => {
                    syn::parse::<syn::Block>(
                        quote!(
                            {
                                let _self = self;
                                let signal_handler_id = self.#child().connect_closure(
                                    stringify!(#signal),
                                    false,
                                    closure_local!(@watch _self => move |b: gtk::Widget| { _self.#handler_name(b.downcast().unwrap()) }),
                                );
                            }
                        )
                        .into(),
                    )
                    .unwrap()
                },
            };

            blocks.push(block);
        }
    }
    blocks
}

pub fn get_signal_connectors(fields: &Punctuated<Field, Comma>) -> Vec<ItemFn> {
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
                        fn #connector_ident(&self, f: impl Fn(&Self) + 'static) -> glib::SignalHandlerId {
                            self.connect_closure(
                                stringify!(#name),
                                false,
                                closure_local!(move |s: Self| {
                                    f(&s);
                                }),
                            )
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

pub fn get_signal_emitters(fields: &Punctuated<Field, Comma>) -> Vec<ItemFn> {
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
