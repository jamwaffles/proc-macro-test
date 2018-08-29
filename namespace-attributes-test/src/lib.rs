#![feature(use_extern_macros)]
#![feature(custom_attribute)]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate namespace_attributes_internals;
extern crate namespace_attributes_macros;

use namespace_attributes_internals::EventData;
use namespace_attributes_macros::ns_test;

#[derive(Serialize)]
struct EventA;

#[derive(Serialize)]
struct EventB;

#[derive(Serialize)]
struct NsEventC;

#[derive(Serialize)]
#[ns_test(namespace = "test_ns")]
#[serde(tag = "type")]
enum Events {
    EventA(EventA),
    EventB(EventB),
    #[ns_test(namespace = "remote_ns")]
    NsEventC(NsEventC),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_string;

    #[test]
    fn it_works() {
        let evt = Events::EventA(EventA);

        let json = to_string(&evt);

        println!("{:?}", json);

        assert_eq!(2 + 2, 4);
    }
}
