// SPDX-License-Identifier: GPL-3.0-or-later

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse,
    punctuated::Punctuated,
    token::{At, Comma},
    Ident, Path, Token,
};

pub struct WidgetMacroArgs {
    pub store: Option<Ident>,
    pub extends_token: Ident,
    pub extends: Path,
    pub implements_token: Option<Ident>,
    pub implements: Punctuated<Path, Comma>,
}

impl std::fmt::Debug for WidgetMacroArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WidgetMacroArgs")
            .field("store", &self.store)
            .field("extends_token", &self.extends_token)
            .field(
                "extends",
                &self
                    .extends
                    .segments
                    .iter()
                    .map(|s| format!("{}", s.ident))
                    .collect::<Vec<String>>()
                    .join("::"),
            )
            .field("implements_token", &self.implements_token)
            .field(
                "implements",
                &self
                    .implements
                    .iter()
                    .map(|p| {
                        p.segments
                            .iter()
                            .map(|s| format!("{}", s.ident))
                            .collect::<Vec<String>>()
                            .join("::")
                    })
                    .collect::<Vec<String>>(),
            )
            .finish()
    }
}

impl Parse for WidgetMacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let store = if input.lookahead1().peek(Token![@]) {
            let _token: At = input.parse().unwrap();
            Some(input.parse()?)
        } else {
            None
        };

        let extends_token = input.parse()?;
        let extends = input.parse()?;
        let mut implements_token = None;
        let mut implements = Punctuated::new();

        if input.lookahead1().peek(Ident) {
            implements_token = Some(input.parse()?);
            implements = Punctuated::parse_terminated(input).unwrap()
        } else {
            implements.push(syn::parse(TokenStream::from(quote! { gtk::Widget })).unwrap());
            implements.push(syn::parse(TokenStream::from(quote! { gtk::Accessible })).unwrap());
            implements.push(syn::parse(TokenStream::from(quote! { gtk::Buildable })).unwrap());
            implements
                .push(syn::parse(TokenStream::from(quote! { gtk::ConstraintTarget })).unwrap());
            implements.push(syn::parse(TokenStream::from(quote! { gtk::Orientable })).unwrap());
        }
        Ok(Self {
            store,
            extends_token,
            extends,
            implements_token,
            implements,
        })
    }
}
