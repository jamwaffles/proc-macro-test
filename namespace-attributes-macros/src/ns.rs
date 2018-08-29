use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::__rt::TokenTree::Group;
use std::collections::HashMap;
use std::iter::repeat;
use std::str::FromStr;
use std::string::ToString;
use syn::{Attribute, Data, DeriveInput};

fn get_namespace_from_attributes(input: &Vec<Attribute>) -> Option<String> {
    input
        .iter()
        .filter_map(|attr| {
            let ident = attr
                .path
                .segments
                .iter()
                .nth(0)
                .expect("Ident")
                .ident
                .to_string();
            if ident == "ns_test" {
                if let Some(Group(g)) = attr.clone().tts.into_iter().next() {
                    g.stream()
                        .into_iter()
                        .nth(2)
                        .map(|namespace| namespace.to_string().trim_matches('"').into())
                } else {
                    None
                }
            } else {
                None
            }
        }).next()
}

pub fn expand_derive_namespace(parsed: &DeriveInput) -> TokenStream {
    let default_namespace = get_namespace_from_attributes(&parsed.attrs)
        .expect("Namespace attribute must be provided at the enum level");

    let variants: HashMap<String, String> = match parsed.data {
        Data::Enum(ref body) => body
            .variants
            .iter()
            .map(|variant| {
                let variant_namespace_override = get_namespace_from_attributes(&variant.attrs);

                let variant_namespace =
                    variant_namespace_override.unwrap_or(default_namespace.clone());

                (variant.ident.to_string(), variant_namespace)
            }).collect(),
        _ => panic!("Namespace can only be derived on enums"),
    };

    let enum_name = parsed.clone().ident.into_token_stream();
    let enum_names = repeat(&enum_name);

    let variant_names = variants
        .keys()
        .map(|k| TokenStream::from_str(k).expect("Variant name"));

    let namespaced_variants = variants.iter().map(|(ident, ns)| {
        TokenStream::from_str(&format!("\"{}.{}\"", ns, ident)).expect("Variant name")
    });

    let out = quote!{
        extern crate namespace_attributes_internals as _internals;

        impl _internals::EventData for #enum_name {
            fn namespaced_type(&self) -> &'static str {
                match self {
                    #(#enum_names::#variant_names(_) => #namespaced_variants,)*
                }
            }
        }
    };

    out
}
