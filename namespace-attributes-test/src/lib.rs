#![feature(use_extern_macros)]
#![feature(custom_attribute)]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate namespace_attributes_internals;
#[macro_use]
extern crate namespace_attributes_macros;

use namespace_attributes_internals::EventData;

#[derive(Serialize)]
struct EventA;

#[derive(Serialize)]
struct EventB;

#[derive(Serialize)]
struct NsEventC;

#[derive(Serialize, EventData)]
#[ns_test(namespace = "test_ns")]
#[serde(tag = "type")]
enum Events {
    EnumEventA(EventA),
    EnumEventB(EventB),
    #[ns_test(namespace = "remote_ns")]
    #[serde(rename = "something")]
    EnumNsEventC(NsEventC),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_string;

    #[test]
    fn it_works() {
        let evt = Events::EnumEventA(EventA);

        let json = to_string(&evt);

        println!("{:?}", json);

        assert_eq!(2 + 2, 4);
    }
}
