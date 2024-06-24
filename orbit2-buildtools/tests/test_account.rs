mod common;
#[test]
fn test_generate_account() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_path = tempdir::TempDir::new("test_account")?.into_path();
    let idl_path = tmp_path.join("account.idl");
    std::fs::write(
        idl_path.clone(),
        "interface Account {
      void deposit (in unsigned long amount);
      void withdraw (in unsigned long amount);
      readonly attribute long balance;
   };",
    )?;

    // These would be set by the cargo build framework
    common::set_env_vars(&tmp_path);

    let r = orbit2_buildtools::CommonBuilder::new("Account")
        .idl_file(&idl_path)
        .out_path(&tmp_path)
        .generate()?;

    assert_eq!(
        r.binding_file.as_os_str(),
        tmp_path.join("Account_binding.rs").as_os_str()
    );

    let syntax = common::parse_file(&r.binding_file)?;

    dbg!(&syntax);

    let types = common::type_names(&syntax);

    assert_eq!(types, vec!["Account"]);

    let foreign_fns = common::foreign_fns(&syntax);
    assert_eq!(
        foreign_fns,
        vec![
            "Account__get_balance",
            "Account_deposit",
            "Account_withdraw"
        ]
    );

    Ok(())
}
