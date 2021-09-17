#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(feature = "bindgen"))]
include!("../gen.rs");

#[cfg(feature = "generate")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
