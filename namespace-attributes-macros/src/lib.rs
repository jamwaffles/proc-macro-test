#![feature(custom_attribute)]
#![feature(proc_macro_non_items)]
#![feature(proc_macro_quote)]

extern crate quote;
#[macro_use]
extern crate proc_macro;
extern crate namespace_attributes_internals;
extern crate syn;

use proc_macro::quote;
use proc_macro::TokenStream;
use proc_macro::TokenTree;
use quote::ToTokens;
use quote::__rt::TokenTree::Group;
use std::collections::HashMap;
use std::string::ToString;
use syn::Attribute;
use syn::Data;
use syn::DeriveInput;
use syn::Fields;
use syn::FieldsUnnamed;

fn get_namespace_from_attributes(input: &Vec<Attribute>) -> Option<String> {
    println!("DBG {:#?}", input);

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

#[proc_macro_derive(EventData, attributes(namespace))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let parsed: DeriveInput = syn::parse(input.clone()).unwrap();

    let default_namespace = get_namespace_from_attributes(&parsed.attrs)
        .expect("Namespace attribute must be provided at the enum level");

    println!("DEFAULT NS {:?}", default_namespace);

    // println!("TOKENS {:#?}", parsed);

    let _variants = if let Data::Enum(body) = parsed.data {
        for variant in body.variants {
            println!("VARIANT {:?}", variant.ident);
            // let fields = if let Fields::Unnamed(f) = variant.fields {
            //     for thing in f.unnamed {
            //         // println!("    FIELD {:#?}", thing);
            //     }
            //     Some(())
            // } else {
            //     None
            // };

            // Find #[ns_test(namespace = "foo")]
            let variant_namespace_override = get_namespace_from_attributes(&variant.attrs);

            let variant_namespace = variant_namespace_override.unwrap_or(default_namespace.clone());

            println!("    VARIANT NS {:?}", variant_namespace);
        }

        ()
    } else {
        ()
    };

    let ident = parsed.ident.into_token_stream();

    quote!{
        extern crate namespace_attributes_internals as _internals;

        impl _internals::EventData for $ident {
            fn get_namespace_and_type(&self) -> String {
                match self {
                    _ => String::new()
                }
            }
        }
    }
}

// #[proc_macro_attribute]
// pub fn ns_test(attr: TokenStream, content: TokenStream) -> TokenStream {
//     match &attr.clone().into_iter().nth(2) {
//         Some(TokenTree::Literal(ref token)) => {
//             let t_s = token.to_string();
//             let ns = t_s.trim_matches('"');
//             println!("NAMESPACE {:?}", ns);

//             let parsed: DeriveInput = syn::parse(content.clone()).unwrap();

//             println!("CONTENT {:#?}", parsed);

//             let ident = parsed.ident.into_token_stream();

//             quote!{
//                 extern crate namespace_attributes_internals;

//                 use namespace_attributes_internals::EventData;

//                 impl EventData for $ident {

//                 }
//             }
//         }

//         bla => {
//             println!("AW {:?}", bla);

//             content
//         }
//     }
// }
