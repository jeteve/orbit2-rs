use std::path::{Path, PathBuf};

fn main() {
    //assert_eq!(env::var("HOST"), Ok("foo".into()));

    let lib = pkg_config::Config::new()
        .atleast_version("2.14.19")
        .print_system_cflags(true)
        .probe("ORBit-2.0")
        .expect("Cannot find ORBit-2.0 with pkg_config");

    let includes = lib
        .include_paths
        .iter()
        .map(|p| format!("-I{}", p.display()))
        .collect::<Vec<_>>();

    // Global corba bindings.
    let core_bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(includes.clone()) // https://stackoverflow.com/questions/64390316/problems-linking-header-files-with-rust-bindgen
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("CORBA_free")
        .allowlist_function("PortableServer_POA_activate_object")
        .allowlist_function("PortableServer_POA_servant_to_reference")
        .allowlist_function("CORBA_exception_free")
        .allowlist_function("CORBA_exception_id")
        .allowlist_function("CORBA_exception_init")
        .allowlist_function("CORBA_ORB_init")
        .allowlist_function("CORBA_free")
        .allowlist_function("CORBA_Object_duplicate")
        .allowlist_function("CORBA_Object_release")
        .allowlist_function("CORBA_ORB_object_to_string")
        .allowlist_function("CORBA_ORB_string_to_object")
        .allowlist_function("CORBA_ORB_resolve_initial_references")
        .allowlist_function("PortableServer_POA__get_the_POAManager")
        .allowlist_function("PortableServer_POAManager_activate")
        .allowlist_function("CORBA_Object_release")
        .allowlist_function("CORBA_ORB_run")
        .allowlist_type("CORBA_exception_type")
        .allowlist_type("CORBA_Object")
        .allowlist_item("CORBA_OBJECT_NIL")
        .allowlist_item("NULL")
        .allowlist_type("PortableServer_ServantBase__epv")
        .allowlist_type("PortableServer_POAManager")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    core_bindings
        .write_to_file(out_path.join("core_bindings.rs"))
        .expect("Couldn't write bindings!");

    // Echo service binding. Do this for every service

    // For clients and server
    let echo_binding = bindgen::Builder::default()
        .header("wrapper_echo.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(includes.clone())
        .allowlist_recursively(false)
        .allowlist_function("Echo_echoString")
        .allowlist_type("Echo")
        .generate()
        .expect("Unable to generate bindings");
    echo_binding
        .write_to_file(out_path.join("echo_bindings.rs"))
        .expect("Couldnt write echo_bindings.rs ");

    // Only for server
    let echo_binding_impl = bindgen::Builder::default()
        .header("wrapper_echo_impl.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(includes.clone())
        .allowlist_recursively(false)
        .allowlist_function("POA_Echo__fini")
        .allowlist_function("POA_Echo__init")
        .allowlist_type("POA_Echo__epv")
        .allowlist_type("POA_Echo__vepv")
        .allowlist_type("POA_Echo")
        .generate()
        .expect("Unable to generate bindings");
    echo_binding_impl
        .write_to_file(out_path.join("echo_bindings_impl.rs"))
        .expect("Couldnt write echo_bindings.rs ");
    // This is for the general (client + server) AND for the server
    cc::Build::new()
        .file("csrc/echo-common.c")
        .file("csrc/echo-skels.c")
        .file("csrc/echo-stubs.c")
        .flag("-Wno-unused-const-variable")
        .flag("-Wno-unused-parameter")
        .includes(Path::new("csrc"))
        .includes(lib.include_paths)
        .compile("echo_idl");
}
