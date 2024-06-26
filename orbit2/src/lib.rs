use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    mem,
    ptr::addr_of_mut,
    str::Utf8Error,
};

use orbit2_sys::core::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    NullInString(std::ffi::NulError),
    Utf8Malformed(Utf8Error),
    CorbaException(String),
    EmptyReference,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for Error {}

type Result<R> = std::result::Result<R, Error>;

pub struct CorbaEnvironment {
    ev: CORBA_Environment,
}

impl CorbaEnvironment {
    fn new() -> Self {
        let mut ev: CORBA_Environment = unsafe { mem::zeroed() };
        unsafe { CORBA_exception_init(&mut ev) };
        CorbaEnvironment { ev }
    }

    pub fn as_corba_environment_ptr(&mut self) -> *mut CORBA_Environment {
        addr_of_mut!(self.ev)
    }
    fn with<F, T>(c: F) -> Result<T>
    where
        F: FnOnce(&mut Self) -> Result<T>,
    {
        let mut ev = Self::new();
        let r = c(&mut ev);
        ev.check_error()?;
        r
    }
    fn check_error(&mut self) -> Result<()> {
        match self.ev._major {
            #[allow(non_upper_case_globals)]
            CORBA_exception_type_CORBA_NO_EXCEPTION => Ok(()),
            _ => {
                let error_string = unsafe { CStr::from_ptr(CORBA_exception_id(&mut self.ev)) }
                    .to_str()
                    .map_err(Error::Utf8Malformed)?;
                Err(Error::CorbaException(error_string.to_string()))
            }
        }
    }
}

struct ArgCV(i32, Vec<*mut i8>);
impl ArgCV {
    fn new<S>(args: &[S]) -> Self
    where
        S: AsRef<str>,
    {
        let argc = i32::try_from(args.len()).expect("Too many args"); // Downcast
        let mut argv = args
            .iter()
            .map(|s| CString::new(s.as_ref()).unwrap_or_default())
            .map(|cs| cs.into_raw())
            .collect::<Vec<_>>();
        argv.reserve(1); // Reserve 1 more, to avoid this to not be allocated, even when args are empty.
        Self(argc, argv)
    }
}
impl Drop for ArgCV {
    fn drop(&mut self) {
        // All CString from the raw pointers to get the memory back.
        unsafe {
            let all_ctrs = self
                .1
                .iter()
                .map(|&r| CString::from_raw(r))
                .collect::<Vec<_>>();
            drop(all_ctrs);
        }
    }
}

pub struct CorbaCharPtr(*mut CORBA_char);
impl CorbaCharPtr {
    pub fn new<S>(s: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let cs = CString::new(s.as_ref()).map_err(Error::NullInString)?;
        Ok(Self(cs.into_raw()))
    }
}

// Meh some CORBA functions copy the pointer so we cannot drop that like all the time.
/* impl Drop for CorbaCharPtr {
    fn drop(&mut self) {
        assert!(!self.0.is_null());
        drop(unsafe { CString::from_raw(self.0) })
    }
} */

impl From<CorbaCharPtr> for *mut CORBA_char {
    fn from(value: CorbaCharPtr) -> Self {
        value.0
    }
}
impl From<CorbaCharPtr> for *const CORBA_char {
    fn from(value: CorbaCharPtr) -> Self {
        value.0
    }
}

// The original Orb MUST outlive this object.
pub struct CorbaObject<O> {
    o: CORBA_Object,
    marker: PhantomData<O>,
}

impl<O> Drop for CorbaObject<O> {
    fn drop(&mut self) {
        CorbaEnvironment::with(|e| {
            unsafe { CORBA_Object_release(self.o, &mut e.ev) };
            Ok(())
        })
        .expect("Cannot destroy Object");
    }
}

impl<O> CorbaObject<O> {
    pub fn with<F, T>(&mut self, c: F) -> Result<T>
    where
        F: FnOnce(&mut Self, &mut CorbaEnvironment) -> Result<T>,
    {
        CorbaEnvironment::with(|e| c(self, e))
    }

    pub fn as_corba_object(&self) -> CORBA_Object {
        self.o
    }
}

pub struct CorbaORB(CORBA_ORB);

//"orbit-local-orb"
impl CorbaORB {
    pub fn new<S>(identifier: S, args: &[S]) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let mut argcv = ArgCV::new(args);
        let orb = CorbaEnvironment::with(|e| unsafe {
            Ok(CORBA_ORB_init(
                addr_of_mut!(argcv.0),
                argcv.1.as_mut_ptr(),
                CorbaCharPtr::new(identifier)?.into(),
                &mut e.ev,
            ))
        })?;
        Ok(Self(orb))
    }

    pub fn get_name_service(&self) -> Result<CorbaObject<CosNaming_NamingContext>> {
        CorbaEnvironment::with(|e| {
            let char_ptr = CorbaCharPtr::new("NameService")?.into();
            let naming_service =
                unsafe { CORBA_ORB_resolve_initial_references(self.0, char_ptr, &mut e.ev) };
            Ok(CorbaObject::<CosNaming_NamingContext> {
                o: naming_service,
                marker: PhantomData,
            })
        })
    }

    pub fn import_object<O, S>(&self, reference: S) -> Result<CorbaObject<O>>
    where
        S: AsRef<str>,
    {
        let ref_str = reference.as_ref();
        if ref_str.is_empty() {
            return Err(Error::EmptyReference);
        }

        let char_ptr = CorbaCharPtr::new(ref_str)?.into();
        let corba_object: CORBA_Object = CorbaEnvironment::with(|e| {
            Ok(unsafe { CORBA_ORB_string_to_object(self.0, char_ptr, &mut e.ev) })
        })?;

        Ok(CorbaObject::<O> {
            o: corba_object,
            marker: PhantomData,
        })
    }
}

impl Drop for CorbaORB {
    fn drop(&mut self) {
        CorbaEnvironment::with(|e| {
            unsafe { CORBA_ORB_destroy(self.0, &mut e.ev) };
            Ok(())
        })
        .expect("Cannot destroy Corba");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_new_env() {
        let mut ev = CorbaEnvironment::new();
        assert!(ev.check_error().is_ok())
    }

    #[test]
    fn test_with_env() {
        let r: Result<()> =
            CorbaEnvironment::with(|_| Err(Error::CorbaException("Boudin".to_owned())));
        assert!(r.is_err());
    }

    #[serial]
    #[test]
    fn test_no_args_orb() {
        let orb = CorbaORB::new("some-orb", &[]);
        assert!(orb.is_ok());
    }

    #[serial]
    #[test]
    fn test_some_args_orb() {
        let orb = CorbaORB::new("some-orb", &["--foo", "bar"]);
        assert!(orb.is_ok());
    }

    #[serial]
    #[test]
    fn test_import_object() {
        let ior = "IOR:010000000d00000049444c3a4563686f3a312e3000000000030000000054424f540000000101020005000000554e4958000000000a0000006c6f63616c686f73740000002b0000002f746d702f6f726269742d7673636f64652f6c696e632d383136642d302d633633346661643363333862000000000000caaedfba58000000010102002b0000002f746d702f6f726269742d7673636f64652f6c696e632d383136642d302d6336333466616433633338620000000000001c000000000000002e2634103549e8a8c22b28282828282801000000be0963b701000000480000000100000002000000050000001c000000000000002e2634103549e8a8c22b28282828282801000000be0963b701000000140000000100000001000105000000000901010000000000";
        let orb = CorbaORB::new("some-orb", &[]).unwrap();
        let obj = orb.import_object::<CORBA_Object, _>(ior);

        assert!(obj.is_ok());
    }

    #[serial]
    #[test]
    fn test_name_service() {
        let orb = CorbaORB::new("some-orb", &[]).unwrap();
        let ns = orb.get_name_service();
        assert!(ns.is_ok());
    }
}
