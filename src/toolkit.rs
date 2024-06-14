use super::*;
use log::warn;
use std::ffi::{CStr, CString};

pub fn raised_exception(ev: &CORBA_Environment) -> bool {
    ev._major != CORBA_exception_type_CORBA_NO_EXCEPTION
}

pub fn raised_exception_is_a(ev: &mut CORBA_Environment, ex: &str) -> bool {
    if !raised_exception(ev) {
        false
    } else {
        dbg!(
            (unsafe { CStr::from_ptr(dbg!(CORBA_exception_id(ev))) })
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
    })
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
