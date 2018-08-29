use ns::get_namespace_from_attributes;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{DataStruct, DeriveInput};

pub fn derive_struct(parsed: &DeriveInput, body: &DataStruct) -> TokenStream {
    let struct_namespace = get_namespace_from_attributes(&parsed.attrs)
        .expect("Namespace attribute must be provided at the struct level");

    let item_ident = parsed.clone().ident.into_token_stream();

    let namespaced_ident = format!("\"{}.{}\"", struct_namespace, item_ident);

    let derive_thing = "#[derive(Deserialize, Debug)]";
    let serde_rename = "#[serde(rename = \"type\")]";
    let test_name = format!("{}DeserializeTest", item_ident);

    let ns = format!("{}", struct_namespace);
    let ty = format!("{}", item_ident);

    let body = body.clone().fields.into_token_stream();

    let out = quote!{


        impl namespace_attributes_internals::EventData for #item_ident {
            fn namespaced_type(&self) -> &'static str {
                #namespaced_ident
            }
        }



        impl<'de> Deserialize<'de> for #item_ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                #[derive(Deserialize, Debug)]
                struct Helper {
                    #[serde(rename = "type")]
                    pub event_type_and_namespace: Option<String>,
                    pub event_type: Option<String>,
                    pub event_namespace: Option<String>,
                }

                struct Output #body;

                impl From<Output> for #item_ident {
                    fn from(out: Self) -> #item_ident {
                        #item_ident {
                            thing: 100
                        }
                    }
                }

                let v = JsonValue::deserialize(deserializer).map_err(de::Error::custom)?;

                let mut type_helper = Helper::deserialize(&v).map_err(de::Error::custom)?;

                // Map old-style event to new-style if new-style is not defined
                if let Some(ref ns_and_ty) = &type_helper.event_type_and_namespace {
                    if type_helper.event_type.is_none() && type_helper.event_namespace.is_none() {
                        let parts: Vec<String> = ns_and_ty.clone().split('.').map(|part| String::from(part)).collect();

                        type_helper.event_namespace = Some(parts[0].clone());
                        type_helper.event_type = Some(parts[1].clone());
                    }
                }

                match (&type_helper.event_namespace, &type_helper.event_type) {
                    (Some(ref ns), Some(ref ty)) => {
                        // println!("MATCH {} {} / {} {} : {}, {}", ns, ty, #ns, #ty, ns == #ns, ty == #ty);

                        if ns != #ns || ty != #ty {
                            Err(de::Error::custom("Data does not match types"))
                        } else {
                            // println!("THING {:?}", v);
                            let out = Output::deserialize(v);

                            out.into()
                        }
                    },
                    _ => Err(de::Error::custom("Could not deserialize event"))
                }
            }
        }
    };

    out
}
