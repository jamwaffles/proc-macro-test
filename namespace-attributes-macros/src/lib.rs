#[macro_use]
extern crate quote;
extern crate syn;

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::DeriveInput;

mod derive_enum;
mod derive_struct;
mod ns;

#[proc_macro_derive(EventData, attributes(ns_test))]
pub fn derive_namespace(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    ns::expand_derive_namespace(&input).into()
}
