#![feature(custom_attribute)]
#![feature(proc_macro_non_items)]
#![feature(proc_macro_quote)]

extern crate quote;
#[macro_use]
extern crate proc_macro;
extern crate namespace_attributes_internals;
extern crate syn;

use namespace_attributes_internals::EventData;
use proc_macro::quote;
use proc_macro::Literal;
use proc_macro::TokenStream;
use proc_macro::TokenTree;
use quote::ToTokens;
use std::str::FromStr;
use syn::buffer::TokenBuffer;
use syn::Attribute;
use syn::Data;
use syn::DeriveInput;
use syn::MetaNameValue;

#[proc_macro_attribute]
pub fn ns_test(attr: TokenStream, content: TokenStream) -> TokenStream {
    match &attr.clone().into_iter().nth(2) {
        Some(TokenTree::Literal(ref token)) => {
            let t_s = token.to_string();
            let ns = t_s.trim_matches('"');
            println!("NAMESPACE {:?}", ns);

            let parsed: DeriveInput = syn::parse(content.clone()).unwrap();

            println!("CONTENT {:#?}", parsed);

            let ident = parsed.ident.into_token_stream();

            quote!{
                extern crate namespace_attributes_internals;

                use namespace_attributes_internals::EventData;

                impl EventData for $ident {

                }
            }
        }

        bla => {
            println!("AW {:?}", bla);

            content
        }
    }
}
// pub fn ns_test(input: TokenStream) -> TokenStream {
//     println!("Neat");

//     input
// }
