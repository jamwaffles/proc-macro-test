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
use quote::ToTokens;
use syn::DeriveInput;

#[proc_macro_derive(EventData, attributes(ns_test))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let parsed: DeriveInput = syn::parse(input.clone()).unwrap();

    println!("TOKENS {:#?}", parsed);

    let ident = parsed.ident.into_token_stream();

    // input
    quote!{
        extern crate namespace_attributes_internals as _internals;

        impl _internals::EventData for $ident {
            fn get_namespace_and_type() -> String {
                String::new()
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
