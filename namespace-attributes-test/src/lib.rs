extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate namespace_attributes_internals;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate namespace_attributes_macros;

use std::fmt;

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::{from_value, Value as JsonValue};
use std::collections::HashMap;

#[derive(EventData, Debug)]
#[ns_test(namespace = "test_ns")]
struct EventA {
    thing: u32,
}

// #[derive(EventData)]
// #[ns_test(namespace = "test_ns")]
// struct EventB {
//     thing: u32,
// }

// #[derive(EventData)]
// #[ns_test(namespace = "test_ns")]
// struct NsEventC {
//     thing: u32,
// }

// #[derive(EventData, Deserialize)]
// #[ns_test(namespace = "test_ns")]
// // #[serde(tag = "type")]
// enum Events {
//     EnumEventA(EventA),
//     EnumEventB(EventB),
//     // #[ns_test(namespace = "remote_ns")]
//     // #[ns_test(rename = "RenamedRemoteEvent")]
//     EnumNsEventC(NsEventC),
// }

#[cfg(test)]
mod tests {
    use super::*;
    use namespace_attributes_internals::EventData;
    use serde_json::from_value;
    use serde_json::to_value;

    #[test]
    fn it_deserializes_events_correctly() {
        #[derive(EventData, Debug)]
        #[ns_test(namespace = "remote_ns_newer")]
        struct TestEvent {
            thing: u32,
        }

        let json = json!({
            "type": "remote_ns.TestEventOld",
            "event_namespace": "remote_ns_newer",
            "event_type": "TestEvent",
            "thing": 100,
        });

        let event = from_value::<TestEvent>(json);

        assert!(event.is_ok());
        assert_eq!(event.unwrap().thing, 100);
    }

    #[test]
    fn it_deserializes_events_with_old_def() {
        #[derive(EventData, Debug)]
        #[ns_test(namespace = "remote_ns")]
        struct TestEvent {
            thing: u32,
        }

        let json = json!({
            "type": "remote_ns.TestEvent",
            "thing": 100,
        });

        let event = from_value::<TestEvent>(json);

        assert!(event.is_ok());
        assert_eq!(event.unwrap().thing, 100);
    }

    #[test]
    fn it_deserializes_events_with_new_def() {
        #[derive(EventData, Debug)]
        #[ns_test(namespace = "remote_ns_newer")]
        struct TestEvent {
            thing: u32,
        }

        let json = json!({
            "event_namespace": "remote_ns_newer",
            "event_type": "TestEvent",
            "thing": 100,
        });

        let event = from_value::<TestEvent>(json);

        assert!(event.is_ok());
        assert_eq!(event.unwrap().thing, 100);
    }

    // #[test]
    // fn it_gets_a_namespaced_struct_type() {
    //     #[derive(EventData)]
    //     #[ns_test(namespace = "some_namespace")]
    //     struct TestStruct {
    //         thing: u32,
    //     }

    //     let thing = TestStruct { thing: 100 };

    //     assert_eq!(thing.namespaced_type(), "some_namespace.TestStruct");
    // }

    // #[test]
    // fn it_serializes_structs() {
    //     #[derive(EventData)]
    //     #[ns_test(namespace = "some_namespace")]
    //     struct TestStruct {
    //         thing: u32,
    //     }

    //     let thing = TestStruct { thing: 100 };

    //     assert_eq!(
    //         to_value(&thing).unwrap(),
    //         json!({
    //             "type": "some_namespace.TestStruct",
    //             "thing": 100
    //         })
    //     );
    // }

    // #[test]
    // fn it_gets_namespaced_event_names() {
    //     let event_a = Events::EnumEventA(EventA { thing: 100 });
    //     let event_b = Events::EnumEventB(EventB { thing: 100 });
    //     let event_c = Events::EnumNsEventC(NsEventC { thing: 100 });

    //     assert_eq!(event_a.namespaced_type(), "test_ns.EnumEventA");
    //     assert_eq!(event_b.namespaced_type(), "test_ns.EnumEventB");
    //     assert_eq!(event_c.namespaced_type(), "remote_ns.EnumNsEventC");
    // }

    // #[test]
    // fn it_serializes_events_with_extra_fields() {
    //     let event = Events::EnumEventA(EventA { thing: 100 });

    //     let json = to_value(&event).unwrap();

    //     assert_eq!(
    //         json,
    //         json!({
    //             "type": "test_ns.EventA",
    //             "event_namespace": "test_ns",
    //             "event_type": "EventA",
    //             "thing": 100,
    //         })
    //     );
    // }

    // #[test]
    // fn it_serializes_events_with_overridden_namespace() {
    //     let event = Events::EnumNsEventC(NsEventC { thing: 100 });

    //     let json = to_value(&event).unwrap();

    //     assert_eq!(
    //         json,
    //         json!({
    //             "type": "remote_ns.RenamedRemoteEvent",
    //             "event_namespace": "remote_ns",
    //             "event_type": "RenamedRemoteEvent",
    //             "thing": 100,
    //         })
    //     );
    // }
}
