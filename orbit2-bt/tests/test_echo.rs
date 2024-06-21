use std::{env, error::Error, fs};

use orbit2_bt::CommonBuilder;
use tempdir::TempDir;

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
    env::set_var("OUT_DIR", tmp_path.as_os_str());
    env::set_var("TARGET", env!("TEST_TARGET"));
    env::set_var("HOST", env!("TEST_TARGET")); // No cross compilation
    env::set_var("OPT_LEVEL", "0");

    let r = CommonBuilder::new("Echo")
        .idl_file(&idl_path)
        .out_path(&tmp_path)
        .generate()?;

    assert_eq!(
        r.binding_file.as_os_str(),
        tmp_path.join("Echo_binding.rs").as_os_str()
    );

    // Parse the file to check the symbols inside
    let bytes = fs::read(r.binding_file)?;
    let binding_code = std::str::from_utf8(&bytes)?;
    let syntax = syn::parse_file(binding_code)?;

    dbg!(&syntax);

    let mut types = syntax
        .items
        .iter()
        .flat_map(|i| match i {
            syn::Item::Type(s) => Some(s),
            _ => None,
        })
        .map(|s| s.ident.to_string())
        //.filter(|s| s.starts_with("Echo"))
        .collect::<Vec<_>>();
    types.sort();

    assert_eq!(types, vec!["Echo"]);

    let mut foreign_fns = syntax
        .items
        .iter()
        .flat_map(|i| match i {
            syn::Item::ForeignMod(m) => Some(m),
            _ => None,
        })
        .flat_map(|fm| &fm.items)
        .flat_map(|i| match i {
            syn::ForeignItem::Fn(ff) => Some(ff),
            _ => None,
        })
        .map(|ff| ff.sig.ident.to_string())
        .collect::<Vec<_>>();

    foreign_fns.sort();
    assert_eq!(foreign_fns, vec!["Echo_echoString"]);

    Ok(())
}
