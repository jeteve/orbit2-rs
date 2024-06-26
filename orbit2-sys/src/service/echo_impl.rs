#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code, unused_variables)]
#![allow(improper_ctypes)]
#![allow(unused_imports)]

use std::{
    ffi::c_void,
    mem,
    ptr::{null, null_mut},
};

use crate::toolkit::charptr_to_string;

use crate::core::*;
use crate::service::echo::Echo;

include!(concat!(env!("OUT_DIR"), "/echo_bindings_impl.rs"));

/* Need to implement this structure:

typedef struct {
POA_Echo servant;
PortableServer_POA poa;
   /* ------ add private attributes here ------ */
   /* ------ ---------- end ------------ ------ */
} impl_POA_Echo;
*/
#[repr(C)]
#[derive(Debug, Clone)]
pub struct impl_POA_Echo {
    pub servant: POA_Echo,
    pub poa: PortableServer_POA,
    // And your private attributes here
    pub name: String,
    pub counter: u64,
}

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
static mut impl_Echo_epv: POA_Echo__epv = unsafe { mem::zeroed() };
static mut impl_Echo_base_epv: PortableServer_ServantBase__epv = unsafe { mem::zeroed() };
static mut impl_Echo_vepv: POA_Echo__vepv = unsafe { mem::zeroed() };

pub fn init_global_structs() {
    // See echo-skepimpl.c for C implementation
    unsafe {
        impl_Echo_epv._private = null_mut();
        impl_Echo_epv.echoString = Some(impl_Echo_echoString);

        dbg!(impl_Echo_epv);

        impl_Echo_base_epv._private = null_mut();
        impl_Echo_base_epv.finalize = Some(impl_Echo__destroy);
        impl_Echo_base_epv.default_POA = None;

        dbg!(impl_Echo_base_epv);

        impl_Echo_vepv.Echo_epv = std::ptr::addr_of_mut!(impl_Echo_epv);
        impl_Echo_vepv._base_epv = std::ptr::addr_of_mut!(impl_Echo_base_epv);

        dbg!(impl_Echo_vepv);
    }
}

/*
    Need to implement the following function:
*/

/*
static Echo impl_Echo__create(PortableServer_POA poa, CORBA_Environment *ev)
{
Echo retval;
impl_POA_Echo *newservant;
PortableServer_ObjectId *objid;

newservant = g_new0(impl_POA_Echo, 1);
newservant->servant.vepv = &impl_Echo_vepv;
newservant->poa = (PortableServer_POA) CORBA_Object_duplicate((CORBA_Object)poa, ev);
POA_Echo__init((PortableServer_Servant)newservant, ev);
   /* Before servant is going to be activated all
    * private attributes must be initialized.  */

   /* ------ init private attributes here ------ */
   /* ------ ---------- end ------------- ------ */

objid = PortableServer_POA_activate_object(poa, newservant, ev);
CORBA_free(objid);
retval = PortableServer_POA_servant_to_reference(poa, newservant, ev);

return retval;
}
 */

///
/// # Safety
///  Any null pointer given will cause panic!
///
#[no_mangle]
pub unsafe extern "C" fn impl_Echo__create(
    poa: PortableServer_POA,
    ev: *mut CORBA_Environment,
) -> Echo {
    assert!(!ev.is_null());
    let mut new_subservant: POA_Echo = unsafe { mem::zeroed() };
    new_subservant.vepv = std::ptr::addr_of_mut!(impl_Echo_vepv);

    let newservant = Box::new(impl_POA_Echo {
        servant: new_subservant,
        poa: CORBA_Object_duplicate(poa as CORBA_Object, ev) as PortableServer_POA,
        name: "chris".to_owned(),
        counter: 0,
    });

    //let mut newservant = Box::new(mem::zeroed::<impl_POA_Echo>());
    //newservant.servant.vepv = std::ptr::addr_of_mut!(impl_Echo_vepv);
    //newservant.poa = CORBA_Object_duplicate(poa as CORBA_Object, ev) as PortableServer_POA;
    //newservant.name = "Chris".to_owned();

    let pservant = dbg!(Box::into_raw(newservant));
    POA_Echo__init(pservant as PortableServer_Servant, ev);

    /* ------ init private attributes here ------ */
    /* ------ ---------- end ------------- ------ */

    let obj_id = PortableServer_POA_activate_object(poa, pservant as PortableServer_Servant, ev);
    CORBA_free(dbg!(obj_id) as gpointer);

    PortableServer_POA_servant_to_reference(poa, pservant as PortableServer_Servant, ev)
}

/*
ption<unsafe extern "C" fn(*mut c_void, *const i8, *mut servant::CORBA_Environment_type)>`
found fn item `extern "C" fn(*mut impl_POA_Echo, *mut i8, *mut servant::CORBA_Environment_type)

*/

/// # Safety
/// Any null pointer given to this will panic
#[no_mangle]
pub unsafe extern "C" fn impl_Echo__destroy(
    servant: *mut c_void, // impl_POA_Echo
    ev: *mut CORBA_Environment,
) {
    assert!(!servant.is_null());
    assert!(!ev.is_null());
    let poa_object = (*(servant as *mut impl_POA_Echo)).poa as CORBA_Object;

    CORBA_Object_release(poa_object, ev);

    // Place to do some freeing of stuff yourself,
    // in case you have resources living in the servant.

    // Eventually calls PortableServer_ServantBase__fini
    // which does this:
    POA_Echo__fini(servant as PortableServer_Servant, ev);

    // To actually free the rust side memory.
    #[allow(clippy::from_raw_with_void_ptr)]
    drop(Box::from_raw(dbg!(servant)));
}

///
/// # Safety
/// If you give null pointers to this, it will panic.
#[no_mangle]
pub unsafe extern "C" fn impl_Echo_echoString(
    servant: *mut c_void, //impl_POA_Echo, // impl_POA_Echo,
    input: *const CORBA_char,
    ev: *mut CORBA_Environment,
) {
    assert!(!servant.is_null());
    assert!(!input.is_null());
    assert!(!ev.is_null());
    // What the method does:
    let real_servant = servant as *mut impl_POA_Echo;
    (*real_servant).counter += 1;
    println!(
        "Hello from {} # {}",
        (*real_servant).name,
        (*real_servant).counter
    );
    println!("Received string={:?}", charptr_to_string(input as *mut i8));
}
