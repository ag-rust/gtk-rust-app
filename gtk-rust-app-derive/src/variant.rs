use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Field, Ident, ItemImpl, Token};

pub fn variant_serde_json(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemStruct);

    let fields = match &input.fields {
        syn::Fields::Named(named) => Some(&named.named),
        syn::Fields::Unnamed(_) => None,
        syn::Fields::Unit => None,
    };

    if fields.is_none() {
        panic!("the #[variant] macro is only supported for structs with named fields");
    }
    let fields = fields.unwrap();

    let to_variant_impl = get_to_variant_impl(&input.ident, fields);
    let from_variant_impl = get_from_variant_impl(&input.ident, fields);

    let gen = quote! {
        #input

        #to_variant_impl

        #from_variant_impl

        impl StaticVariantType for Event {
            fn static_variant_type() -> std::borrow::Cow<'static, glib::VariantTy> {
                glib::Variant::static_variant_type()
            }
        }

    };
    TokenStream::from(gen)
}

fn get_to_variant_impl(ident: &Ident, _fields: &Punctuated<Field, Token![,]>) -> ItemImpl {
    syn::parse(TokenStream::from(quote!(
        impl glib::ToVariant for #ident {
            fn to_variant(&self) -> glib::Variant {
                serde_json::to_string(self).unwrap().to_variant()
            }
        }
    )))
    .unwrap()
}

fn get_from_variant_impl(ident: &Ident, _fields: &Punctuated<Field, Token![,]>) -> ItemImpl {
    syn::parse(TokenStream::from(quote!(
        impl glib::FromVariant for #ident {
            fn from_variant(variant: &glib::Variant) -> Option<Self> {
                if let Some(s) = variant.get::<String>() {
                    serde_json::from_str(&s).ok()
                } else {
                    None
                }
            }
        }
    )))
    .unwrap()
}
