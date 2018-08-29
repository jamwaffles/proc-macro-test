use ns::get_namespace_from_attributes;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{DataStruct, DeriveInput};

pub fn derive_struct(parsed: &DeriveInput, _body: &DataStruct) -> TokenStream {
    let struct_namespace = get_namespace_from_attributes(&parsed.attrs)
        .expect("Namespace attribute must be provided at the struct level");

    let item_ident = parsed.clone().ident.into_token_stream();

    let namespaced_ident = format!("\"{}.{}\"", struct_namespace, item_ident);

    let out = quote!{
        impl namespace_attributes_internals::EventData for #item_ident {
            fn namespaced_type(&self) -> &'static str {
                #namespaced_ident
            }
        }
    };

    out
}
