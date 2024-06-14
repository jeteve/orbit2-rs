use super::*;
use log::warn;
use std::{
    default,
    ffi::{CStr, CString},
    ptr::{null, null_mut},
};

pub fn raised_exception(ev: &CORBA_Environment) -> bool {
    ev._major != CORBA_exception_type_CORBA_NO_EXCEPTION
}

pub fn raised_exception_is_a(ev: &mut CORBA_Environment, ex: &str) -> bool {
    if !raised_exception(ev) {
        false
    } else {
        dbg!(
            unsafe { CStr::from_ptr(dbg!(CORBA_exception_id(ev))) }
                == CString::new(ex).unwrap_or_default().as_c_str()
        )
    }
}

/// panic! if an exception is in the given environment
pub fn abort_if_exception(ev: &mut CORBA_Environment, msg: &str) -> () {
    if !raised_exception(ev) {
        return;
    }
    panic!("{} {:?}", msg, unsafe {
        CStr::from_ptr(CORBA_exception_id(ev))
    })
}

pub fn ignore_if_exception(ev: &mut CORBA_Environment, msg: &str) -> () {
    if !raised_exception(ev) {
        return;
    }
    warn!("{} {:?}", msg, unsafe {
        CStr::from_ptr(CORBA_exception_id(ev))
    });
    unsafe { CORBA_exception_free(ev) };
}

pub fn charptr_to_string(value: *mut CORBA_char) -> Option<String> {
    if value.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(value) }
                .to_owned()
                .to_str()
                .unwrap_or("INVALID_UTF8")
                .to_string(),
        )
    }
}

pub fn exception_string(ev: &mut CORBA_Environment) -> Option<String> {
    if raised_exception(ev) {
        let cs = unsafe { CStr::from_ptr(dbg!(CORBA_exception_id(ev))) };
        Some(cs.to_owned().to_str().expect("Arg, not UTF-8").to_string())
    } else {
        None
    }
}

pub fn export_object(
    orb: CORBA_ORB,
    servant: CORBA_Object,
    ev: &mut CORBA_Environment,
) -> Result<String, String> {
    let string = unsafe { CORBA_ORB_object_to_string(orb, servant, ev) };
    if raised_exception(ev) {
        Err(exception_string(ev).unwrap_or_default())
    } else {
        charptr_to_string(string).ok_or(format!("Cannot turn {:?} into String", string))
    }
}

pub fn import_object(
    orb: CORBA_ORB,
    s: &str,
    ev: &mut CORBA_Environment,
) -> Result<CORBA_Object, String> {
    if s.len() == 0 {
        return Err("Empty s".to_owned());
    }

    let s = CString::new(s).map_err(|e| format!("{}", e))?.into_raw();
    let obj = unsafe { CORBA_ORB_string_to_object(orb, s, ev) };
    // Make sure the CString is freed.
    let _ = unsafe { CString::from_raw(s) };

    if raised_exception(ev) {
        return Err(exception_string(ev).unwrap_or_default());
    }

    if obj.is_null() {
        Err("Returned object is null".to_owned())
    } else {
        Ok(obj)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    fn new_ev() -> CORBA_Environment {
        let mut ev = unsafe { std::mem::zeroed() };
        unsafe { CORBA_exception_init(&mut ev) };
        ev
    }

    #[test]
    fn test_raised_exception() {
        let mut ev = new_ev();

        ev._major = CORBA_exception_type_CORBA_SYSTEM_EXCEPTION;
        assert!(raised_exception(&ev));

        ev._major = CORBA_exception_type_CORBA_NO_EXCEPTION;
        assert!(!raised_exception(&ev));
    }

    #[test]
    fn test_raised_exception_is_a() {
        let mut ev = new_ev();
        // ev._major = CORBA_exception_type_CORBA_NO_EXCEPTION;
        assert!(!raised_exception_is_a(&mut ev, "sausage"));
    }
}
