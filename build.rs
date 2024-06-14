use std::path::Path;
use std::path::PathBuf;

fn main() {
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
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(includes.clone()) // https://stackoverflow.com/questions/64390316/problems-linking-header-files-with-rust-bindgen
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Echo service binding. Do this for every service
    let echo_binding = bindgen::Builder::default()
        .header("wrapper_echo.h")
        .clang_args(includes.clone())
        .generate()
        .expect("Unable to generate bindings");
    echo_binding
        .write_to_file(out_path.join("echo_bindings.rs"))
        .expect("Couldnt write echo_bindings.rs ");

    cc::Build::new()
        .file("csrc/echo-common.c")
        .file("csrc/echo-skels.c")
        .file("csrc/echo-stubs.c")
        .includes(Path::new("csrc"))
        .includes(lib.include_paths)
        .compile("echo_idl");
}
