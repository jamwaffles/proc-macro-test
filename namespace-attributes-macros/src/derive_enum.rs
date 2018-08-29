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

    let item_ident = parsed.clone().ident.into_token_stream();
    let item_idents = repeat(&item_ident);

    let variant_names = variants
        .keys()
        .map(|k| TokenStream::from_str(k).expect("Variant name"));

    let namespaced_variants = variants.iter().map(|(ident, ns)| {
        TokenStream::from_str(&format!("\"{}.{}\"", ns, ident)).expect("Variant name")
    });

    let out = quote!{
        impl namespace_attributes_internals::EventData for #item_ident {
            fn namespaced_type(&self) -> &'static str {
                match self {
                    #(#item_idents::#variant_names(_) => #namespaced_variants,)*
                }
            }
        }
    };

    out
}
