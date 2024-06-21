use std::env;

pub(crate) fn type_names(syntax: &syn::File) -> Vec<String> {
    let mut types = syntax
        .items
        .iter()
        .flat_map(|i| match i {
            syn::Item::Type(s) => Some(s),
            _ => None,
        })
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>();
    types.sort();

    types
}

pub(crate) fn foreign_fns(syntax: &syn::File) -> Vec<String> {
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
    foreign_fns
}

pub(crate) fn set_env_vars(tmp_path: &std::path::Path) -> () {
    // These would be set by the cargo build framework
    env::set_var("OUT_DIR", tmp_path.as_os_str());
    env::set_var("TARGET", env!("TEST_TARGET"));
    env::set_var("HOST", env!("TEST_TARGET")); // No cross compilation
    env::set_var("OPT_LEVEL", "0");
}

pub(crate) fn parse_file(path: &std::path::Path) -> syn::Result<syn::File> {
    let content = std::fs::read_to_string(path).expect("Unable to read file");
    syn::parse_file(&content)
}
