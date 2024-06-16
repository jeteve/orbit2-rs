use std::{
    error::Error,
    fs::{read_to_string, File},
    mem,
    ptr::addr_of_mut,
};

use orbit2_sys::{
    echo::{servant::Echo, Echo_echoString},
    toolkit::{abort_if_exception, import_object, string_to_corba_char, vecs_to_argcv},
    CORBA_Environment, CORBA_ORB_init, CORBA_exception_init, CORBA_ORB,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("This is echo client");

    let mut ev: CORBA_Environment = unsafe { mem::zeroed() };
    unsafe { CORBA_exception_init(&mut ev) };

    let (mut argc, mut argv) = vecs_to_argcv(&vec!["--foo".to_owned()]);
    let global_orb: CORBA_ORB = unsafe {
        CORBA_ORB_init(
            addr_of_mut!(argc),
            argv.as_mut_ptr(),
            string_to_corba_char("orbit-local-orb"),
            &mut ev,
        )
    };
    abort_if_exception(&mut ev, "cannot ORB Init");

    let ref_data = read_to_string("echo.ref")?;

    let echo_service = import_object(global_orb, &ref_data, &mut ev)? as Echo;
    abort_if_exception(&mut ev, "cannot import object");

    unsafe { Echo_echoString(echo_service, string_to_corba_char("Sausage"), &mut ev) };
    abort_if_exception(&mut ev, "cannot call echo_service");

    Ok(())
}
