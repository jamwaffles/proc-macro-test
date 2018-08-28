#![feature(use_extern_macros)]
#![feature(custom_attribute)]

#[macro_use]
extern crate namespace_attributes_macros;

use namespace_attributes_macros::ns_test;

struct EventA;
struct EventB;
struct NsEventC;

#[ns_test(namespace = "test_ns")]
enum Events {
    EventA(EventA),
    EventB(EventB),
    #[ns_test(namespace = "remote_ns")]
    NsEventC(NsEventC),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
