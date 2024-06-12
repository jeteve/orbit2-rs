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

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(includes) // https://stackoverflow.com/questions/64390316/problems-linking-header-files-with-rust-bindgen
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
