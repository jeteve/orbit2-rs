#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code, unused_variables)]
#![allow(improper_ctypes)]
#![allow(unused_imports)]

use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};
include!(concat!(env!("OUT_DIR"), "/echo_bindings_impl.rs"));

/*

Need to implement the initialisation of this variable:
static POA_Echo__epv impl_Echo_epv = {
NULL, /* _private */
(gpointer)&impl_Echo_echoString,
};

And this one:
static PortableServer_ServantBase__epv impl_Echo_base_epv = {
NULL,             /* _private data */
(gpointer) & impl_Echo__destroy, /* finalize routine */
NULL,             /* default_POA routine */
};



*/

pub fn init_global_structs() -> () {
    // See echo-skepimpl.c for C implementation
    unsafe {
        impl_Echo_epv._private = null_mut();
        impl_Echo_epv.echoString = Some(impl_Echo_echoString);

        impl_Echo_base_epv._private = null_mut();
        impl_Echo_base_epv.finalize = Some(impl_Echo__destroy);
        impl_Echo_base_epv.default_POA = None;

        impl_Echo_vepv.Echo_epv = std::ptr::addr_of_mut!(impl_Echo_epv);
        impl_Echo_vepv._base_epv = std::ptr::addr_of_mut!(impl_Echo_base_epv);
    }
    todo!()
}

/*
ption<unsafe extern "C" fn(*mut c_void, *const i8, *mut servant::CORBA_Environment_type)>`
found fn item `extern "C" fn(*mut impl_POA_Echo, *mut i8, *mut servant::CORBA_Environment_type)

*/

#[no_mangle]
pub unsafe extern "C" fn impl_Echo__destroy(
    servant: *mut c_void, // impl_POA_Echo
    ev: *mut CORBA_Environment,
) {
    let poa_object = (*(servant as *mut impl_POA_Echo)).poa as CORBA_Object;

    CORBA_Object_release(poa_object, ev);

    // Place to do some freeing.
}

#[no_mangle]
pub unsafe extern "C" fn impl_Echo_echoString(
    servant: *mut c_void, // impl_POA_Echo,
    input: *const CORBA_char,
    ev: *mut CORBA_Environment,
) -> () {
    // What the method does:
    println!("Hello from echo");
}