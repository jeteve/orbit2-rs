use std::path::PathBuf;

use orbit2_buildtools::CommonBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let idl_path = PathBuf::from("static/echo.idl");

    println!("cargo::rerun-if-changed=static/echo.idl");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let r = CommonBuilder::new("Echo")
        .idl_file(&idl_path)
        .out_path(&out_path)
        .generate()?;

    println!(
        "cargo:rustc-env=ECHO_IDL_BINDING={}",
        r.binding_file.as_path().to_str().unwrap()
    );

    println!(
        "cargo::warning=Binding: {}",
        r.binding_file.as_path().to_str().unwrap()
    );

    Ok(())
}
