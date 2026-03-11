extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::Ident as Ident2;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{self, Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(ContextExtension, attributes(tag))]
pub fn context_extension_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(FieldEnumerator, attributes(tag))]
pub fn field_enumerator_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    // Only support structs with named fields
    let fields = if let Data::Struct(s) = input.data {
        if let Fields::Named(named) = s.fields {
            named.named
        } else {
            panic!("FieldEnumerator only works on structs with named fields");
        }
    } else {
        panic!("FieldEnumerator only works on structs");
    };

    let inject_by_tag = collect_injectable_by_tag(&struct_name, fields);

    let tag_macros = inject_by_tag.into_iter().map(|(tag_type, injects)| {
        let enumerate_fields = format_ident!("enumerate_tags_{}_{}", struct_name, tag_type);

        quote! {
            macro_rules! #enumerate_fields {
               ($macro_callback:ident) => {
                     #( #injects )*
               }
            }
        }
    });

    TokenStream::from(quote! {
        #( #tag_macros )*
    })
}

fn collect_injectable_by_tag(
    struct_name: &Ident2,
    fields: syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> HashMap<Ident2, Vec<TokenStream2>> {
    let mut inject_by_tag: HashMap<Ident2, Vec<TokenStream2>> = HashMap::new();

    // // Process fields: remove #[tag(...)] attributes
    fields.into_iter().for_each(|field| {
        let field_name = field
            .ident
            .unwrap_or_else(|| panic!("FieldEnumerator only works on structs with named fields"));

        field.attrs.iter().for_each(|attribute| {
            if !attribute.path().is_ident("tag") {
                return;
            }

            let mut tag_type = Option::None;

            attribute
                .parse_nested_meta(|x| {
                    tag_type = Option::Some(x.path);
                    Result::Ok(())
                })
                .unwrap_or_else(|_| {
                    panic!(
                        "FieldEnumerator attribute #tag(#TagType#) \
                        expects one parameter that represents event for field {}",
                        field_name
                    )
                });

            let tag_type = tag_type.unwrap();
            let tag_type = tag_type.get_ident().unwrap_or_else(|| {
                panic!(
                    "FieldEnumerator attribute #tag(#TagType#) \
                        expects one parameter of type indent that represents event for field {}",
                    field_name
                )
            });

            inject_by_tag.entry(tag_type.clone()).or_default().push(quote! {
                $macro_callback!(#struct_name, #field_name, #tag_type);
            });
        });
    });
    inject_by_tag
}
