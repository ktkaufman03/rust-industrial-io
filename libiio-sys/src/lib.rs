// libiio-sys/src/lib.rs
//
//!
//!

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Temporary
#![allow(dead_code)]

// Bindgen uses u128 on some rare parameters
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/iio_bindings.rs"));