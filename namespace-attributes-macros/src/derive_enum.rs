use ns::get_namespace_from_attributes;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashMap;
use std::iter::repeat;
use std::str::FromStr;
use std::string::ToString;
use syn::{DataEnum, DeriveInput};

pub fn derive_enum(parsed: &DeriveInput, body: &DataEnum) -> TokenStream {
    let default_namespace = get_namespace_from_attributes(&parsed.attrs)
        .expect("Namespace attribute must be provided at the enum level");

    let variants: HashMap<String, String> = body
        .variants
        .iter()
        .map(|variant| {
            let variant_namespace_override = get_namespace_from_attributes(&variant.attrs);

            let variant_namespace = variant_namespace_override.unwrap_or(default_namespace.clone());

            (variant.ident.to_string(), variant_namespace)
        }).collect();

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
