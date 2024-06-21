use std::{error::Error, fs};

use orbit2_bt::CommonBuilder;
use tempdir::TempDir;

mod common;

#[test]
fn test_generate_echo() -> Result<(), Box<dyn Error>> {
    let tmp_path = TempDir::new("test_echo")?.into_path();
    let idl_path = tmp_path.join("echo.idl");
    fs::write(
        idl_path.clone(),
        "interface Echo {
    void echoString(in string input);
};",
    )?;

    // These would be set by the cargo build framework
    common::set_env_vars(&tmp_path);

    let r = CommonBuilder::new("Echo")
        .idl_file(&idl_path)
        .out_path(&tmp_path)
        .generate()?;

    assert_eq!(
        r.binding_file.as_os_str(),
        tmp_path.join("Echo_binding.rs").as_os_str()
    );

    // Parse the file to check the symbols inside
    let syntax = common::parse_file(&r.binding_file)?;

    dbg!(&syntax);

    let types = common::type_names(&syntax);

    assert_eq!(types, vec!["Echo"]);

    let foreign_fns = common::foreign_fns(&syntax);
    assert_eq!(foreign_fns, vec!["Echo_echoString"]);

    Ok(())
}
