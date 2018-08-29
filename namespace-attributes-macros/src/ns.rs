use derive_enum::derive_enum;
use derive_struct::derive_struct;
use proc_macro2::TokenStream;
use quote::__rt::TokenTree::Group;
use std::string::ToString;
use syn::{Attribute, Data, DeriveInput};

pub fn get_namespace_from_attributes(input: &Vec<Attribute>) -> Option<String> {
    input
        .iter()
        .filter_map(|attr| {
            // Look through all attribute annotations
            attr.path
                .segments
                .iter()
                // Filter attributes we're interested in
                .find(|segment| segment.ident.to_string() == "ns_test")
                // Find attribute triples like `namespace = "something"`
                .and_then(|_| {
                    attr.clone().tts.into_iter().find(|tt| match tt {
                        Group(_) => true,
                        _ => false,
                    })
                }).and_then(|tt| match tt {
                    // Get last token of `a = b` triplet
                    Group(g) => g
                        .stream()
                        .into_iter()
                        .nth(2)
                        // Convert to string, strip surrounding quotes
                        .map(|namespace| namespace.to_string().trim_matches('"').into()),
                    _ => None,
                })
        }).next()
}

pub fn expand_derive_namespace(parsed: &DeriveInput) -> TokenStream {
    match parsed.data {
        Data::Enum(ref body) => derive_enum(&parsed, &body),
        Data::Struct(ref body) => derive_struct(&parsed, &body),
        _ => panic!("Namespace can only be derived on enums and structs"),
    }
}
