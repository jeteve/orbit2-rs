mod common;
#[test]
fn test_generate_calculator() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_path = tempdir::TempDir::new("test_calculator")?.into_path();
    let idl_path = tmp_path.join("echo.idl");
    std::fs::write(
        idl_path.clone(),
        "interface Calculator
{
      double add(in double number1, in double number2);
      double sub(in double number1, in double number2);
};",
    )?;

    // These would be set by the cargo build framework
    common::set_env_vars(&tmp_path);

    let r = orbit2_bt::CommonBuilder::new("Calculator")
        .idl_file(&idl_path)
        .out_path(&tmp_path)
        .generate()?;

    assert_eq!(
        r.binding_file.as_os_str(),
        tmp_path.join("Calculator_binding.rs").as_os_str()
    );

    let syntax = common::parse_file(&r.binding_file)?;

    dbg!(&syntax);

    let types = common::type_names(&syntax);

    assert_eq!(types, vec!["Calculator"]);

    let foreign_fns = common::foreign_fns(&syntax);
    assert_eq!(foreign_fns, vec!["Calculator_add", "Calculator_sub"]);

    Ok(())
}
