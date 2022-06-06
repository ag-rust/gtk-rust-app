// SPDX-License-Identifier: GPL-3.0-or-later

use quote::{__private::TokenStream, quote};
use syn::{
    parenthesized,
    parse::Parse,
    parse2,
    punctuated::Punctuated,
    token::{Comma, Dot, Paren},
    Attribute, Field, Ident,
};

use super::attributes::ATTR_SELECTOR;

struct Args {
    pub _parent_token: Paren,
    pub paths: Vec<Punctuated<Ident, Dot>>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let parent_token = parenthesized!(content in input);
        let mut selectors = Vec::new();
        let selector = Punctuated::new();
        selectors.push(selector);
        while !content.is_empty() {
            if content.lookahead1().peek(Comma) {
                let _comma: Comma = content.parse().unwrap();
                selectors.push(Punctuated::new());
                continue;
            }
            if content.lookahead1().peek(Dot) {
                let _dor: Dot = content.parse().unwrap();
                continue;
            }
            let ident: Ident = content.parse().unwrap();
            if let Some(s) = selectors.last_mut() {
                s.push(ident);
            }
        }
        Ok(Self {
            _parent_token: parent_token,
            paths: selectors,
        })
    }
}

pub fn get_selector_bindings(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let mut tokens = TokenStream::new();
    for field in fields {
        if let Some(attr) = get_selector_attr(field) {
            let name = field.ident.as_ref().unwrap();
            let attr_tokens = &attr.tokens;
            let args: Args = parse2(attr_tokens.clone()).unwrap();
            let paths = args.paths;
            let sd = quote!(
                {
                    #[allow(clippy::redundant_closure_call)]
                    let selector_id = store().select(
                        stringify!(#name),
                        |s| {
                            let mut state: State = Default::default();
                            #(
                            let sel = |state: &State| #paths.clone();
                            #paths = sel(s);
                            )*
                            state
                        },
                        glib::clone!(@weak obj => move |state| {
                        <Self as ObjectSubclass>::Type::#name(&obj, state);
                    }));
                    let mut selectors = self.selectors.take();
                    selectors.push(selector_id);
                    self.selectors.set(selectors);
                }
            );
            // println!("{}", sd);
            tokens.extend(sd)
        }
    }
    tokens
}

fn get_selector_attr(field: &Field) -> Option<&Attribute> {
    for attr in &field.attrs {
        if attr.path.is_ident(ATTR_SELECTOR) {
            return Some(attr);
        }
    }
    None
}
