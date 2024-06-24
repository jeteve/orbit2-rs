use std::{
    error::Error,
    fs::{self},
    mem,
    ptr::{addr_of_mut, null_mut},
};

use orbit2_sys::{core::*, service::echo_impl};

use orbit2_sys::{
    service::echo::Echo,
    service::echo_impl::impl_Echo__create,
    toolkit::{abort_if_exception, export_object, string_to_corba_char, vecs_to_argcv},
};

static mut GLOBAL_ORB: CORBA_ORB = null_mut();
static mut ROOT_POA: PortableServer_POA = null_mut();

fn server_init(
    args: &[String],
    orb: *mut CORBA_ORB,
    poa: *mut PortableServer_POA,
    ev: *mut CORBA_Environment,
) {
    assert!(!orb.is_null());
    dbg!(unsafe { *orb });
    assert!(!poa.is_null());
    assert!(!ev.is_null());
    let mut local_ev: CORBA_Environment = unsafe { mem::zeroed() };
    unsafe { CORBA_exception_init(&mut local_ev) };
    dbg!(local_ev);

    let (mut argc, mut argv) = vecs_to_argcv(args);

    dbg!(argc);
    dbg!(&argv);

    unsafe {
        *orb = CORBA_ORB_init(
            addr_of_mut!(argc),
            argv.as_mut_ptr(),
            string_to_corba_char("orbit-local-mt-orb"),
            ev,
        );
        abort_if_exception(ev.as_mut().unwrap(), "Cannot init ORB");
    };

    dbg!(unsafe { *orb });
    assert!(!orb.is_null());
    assert!(!(unsafe { *orb }).is_null());
    unsafe {
        *poa = CORBA_ORB_resolve_initial_references(*orb, string_to_corba_char("RootPOA"), ev)
            as PortableServer_POA;
        abort_if_exception(ev.as_mut().unwrap(), "Cannot resolve_initial_references");
    }

    unsafe {
        let poa_manager = PortableServer_POA__get_the_POAManager(*poa, ev);
        abort_if_exception(ev.as_mut().unwrap(), "Cannot get POAManager");
        assert!(!poa_manager.is_null());
        PortableServer_POAManager_activate(poa_manager, ev);
        abort_if_exception(ev.as_mut().unwrap(), "Cannot activate poa_manager");
        CORBA_Object_release(poa_manager as CORBA_Object, ev);
    }
}

fn server_activate_service(
    _orb: CORBA_ORB,
    rpoa: PortableServer_POA,
    ev: *mut CORBA_Environment,
) -> CORBA_Object {
    let reference: Echo = unsafe { impl_Echo__create(rpoa, ev) };
    abort_if_exception(unsafe { ev.as_mut() }.unwrap(), "Cannot build Echo servant");
    reference
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, I am a server");
    echo_impl::init_global_structs();

    // No need for that. We can do it in Rust land.
    //let filename = string_to_corba_char("echo.ref");
    let mut ev: CORBA_Environment = unsafe { mem::zeroed() };
    unsafe { CORBA_exception_init(&mut ev) };
    abort_if_exception(&mut ev, "Cannot init env");
    dbg!(ev);

    dbg!(unsafe { GLOBAL_ORB });
    dbg!(unsafe { ROOT_POA });

    server_init(
        &["--help".to_owned()],
        unsafe { addr_of_mut!(GLOBAL_ORB) },
        unsafe { addr_of_mut!(ROOT_POA) },
        &mut ev,
    );

    assert!(!unsafe { GLOBAL_ORB }.is_null());
    assert!(!unsafe { ROOT_POA }.is_null());

    // alright server_init is done.
    // time to activate the servant
    let servant = server_activate_service(unsafe { GLOBAL_ORB }, unsafe { ROOT_POA }, &mut ev);
    assert!(!servant.is_null());
    dbg!(servant);

    let ref_string = export_object(unsafe { GLOBAL_ORB }, servant, &mut ev);
    dbg!(&ref_string);
    assert!(ref_string.is_ok());

    fs::write("echo.ref", ref_string.unwrap())?;

    unsafe { CORBA_ORB_run(GLOBAL_ORB, &mut ev) };

    Ok(())
}
