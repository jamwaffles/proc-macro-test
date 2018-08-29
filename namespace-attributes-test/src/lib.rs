extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate namespace_attributes_internals;
#[macro_use]
extern crate namespace_attributes_macros;

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
    use namespace_attributes_internals::EventData;

    #[test]
    fn it_gets_namespaced_event_names() {
        let event_a = Events::EnumEventA(EventA);
        let event_b = Events::EnumEventB(EventB);
        let event_c = Events::EnumNsEventC(NsEventC);

        assert_eq!(event_a.namespaced_type(), "test_ns.EnumEventA");
        assert_eq!(event_b.namespaced_type(), "test_ns.EnumEventB");
        assert_eq!(event_c.namespaced_type(), "remote_ns.EnumNsEventC");
    }
}
