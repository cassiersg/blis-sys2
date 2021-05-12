#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate blis_src as _;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
