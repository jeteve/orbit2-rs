#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code, unused_variables)]
#![allow(improper_ctypes)]
#![allow(unused_imports)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod echo;

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr::addr_of_mut;
    use std::ffi::CString;

    #[test]
    fn base_client() {
        let cfilename = CString::new("echo.ref").unwrap();
        let filename: *const CORBA_char = cfilename.as_ptr();
        let mut ev: CORBA_Environment = unsafe { std::mem::zeroed() };
        unsafe { super::CORBA_exception_init(addr_of_mut!(ev)) };

        let args: Vec<String> = vec![];

        let mut argc = args.len() as i32;

        // This MUST hold the CStrings.
        let mut argv = args
            .into_iter()
            .map(|s| CString::new(s).unwrap_or_default())
            .map(|cs| cs.into_raw())
            .collect::<Vec<_>>();

        let orb_identifier: CORBA_ORBid = CString::new("orbit-local-mt-orb").unwrap().into_raw();

        let orb = unsafe {
            CORBA_ORB_init(
                addr_of_mut!(argc),
                argv.as_mut_ptr(),
                orb_identifier,
                addr_of_mut!(ev),
            )
        };

        //let object = CORBA_ORB_string_to_object(orb, addr_of_mut!(ev));

        // Retake ownership of the raw CSStrings so they can be freed in Rust.
        let _ = argv
            .into_iter()
            .map(|ptr| unsafe { CString::from_raw(ptr) })
            .collect::<Vec<_>>();
        let _ = unsafe { CString::from_raw(orb_identifier) };
    }
}
