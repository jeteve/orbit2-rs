#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code, unused_variables)]
#![allow(improper_ctypes)]
#![allow(unused_imports)]

use crate::core::*;

include!(concat!(env!("OUT_DIR"), "/echo_bindings.rs"));
