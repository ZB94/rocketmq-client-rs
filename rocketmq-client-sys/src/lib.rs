#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_static_lifetimes)]

#[cfg(not(feature = "generate"))]
include!("../gen.rs");

#[cfg(feature = "generate")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
