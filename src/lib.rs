#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;
    use std::{borrow::BorrowMut, ffi::CString};

    #[test]
    fn base_client() {
        let cfilename = CString::new("foo.txt").unwrap();
        let filename: *const super::CORBA_char = cfilename.as_ptr();
        let mut ev: super::CORBA_Environment = unsafe { std::mem::zeroed() };
        unsafe { super::CORBA_exception_init(ptr::addr_of_mut!(ev)) };

        let argc: std::ffi::c_int = 1;
        let mut carg1 = CString::new("bla").unwrap();
        let arg1 = carg1.into_raw();
        // TODO: let orb = super::CORBA_ORB_init(ptr::addr_of_mut!(argc));
    }
}
