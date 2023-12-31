mod po6 {
    include!(concat!(env!("OUT_DIR"), "/po6_rt.rs"));
}

mod bindings {
    #![allow(dead_code)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
