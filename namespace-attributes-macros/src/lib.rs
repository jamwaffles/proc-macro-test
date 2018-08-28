#![feature(custom_attribute)]

extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

struct Container {
    namespace: String,
}

impl Container {
    pub fn from_ast() -> Self {
        Self {
            namespace: "(hardcoded)".into(),
        }
    }
}

struct Field {
    namespace: String,
}

#[proc_macro_attribute]
pub fn ns_test(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    println!("attr: {:?} \n\ntokens: {:?}", attr, tokens);
    tokens
}
// pub fn ns_test(input: TokenStream) -> TokenStream {
//     println!("Neat");

//     input
// }
