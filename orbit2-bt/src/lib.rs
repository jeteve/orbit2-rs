use std::{fs, path::PathBuf};

#[derive(Debug, Default, Clone)]
pub struct CommonBuilder {
    service_name: String,
    out_path: PathBuf,
    idl_file: PathBuf,
    orbit_idl: String,
    includes: Vec<PathBuf>,
}

impl CommonBuilder {
    pub fn new() -> Self {
        let includes = find_orbit2_includes();

        CommonBuilder {
            orbit_idl: "orbit-idl-2".to_owned(),
            includes,
            ..Default::default()
        }
    }

    pub fn service_name(self, service_name: String) -> Self {
        CommonBuilder {
            service_name,
            ..self
        }
    }

    pub fn idl_file(self, idl_file: PathBuf) -> Self {
        CommonBuilder { idl_file, ..self }
    }

    pub fn out_path(self, out_path: PathBuf) -> Self {
        CommonBuilder { out_path, ..self }
    }

    pub fn generate(&self) {
        let cfiles = self.generate_common_ccode();

        let mut cc = cc::Build::new();
        let cc = cfiles.iter().fold(&mut cc, |cc, f| cc.file(f));

        cc.include(self.out_path.clone())
            .includes(self.includes.clone())
            .compile(&format!("{}_common", self.service_name));

        ()
    }

    fn generate_common_ccode(&self) -> Vec<PathBuf> {
        use std::process::Command;
        let output = Command::new(self.orbit_idl.clone())
            .arg(format!(
                "--output-dir={}",
                self.out_path.clone().to_str().expect("Not unicode dirname")
            ))
            .arg(self.idl_file.clone())
            .output()
            .expect("Failed to run IDL");
        assert!(output.status.success());
        let cfiles = fs::read_dir(self.out_path.clone())
            .expect("Can list tmp_path")
            .map(|r| r.expect("Good dir entry"))
            .map(|d| d.path())
            .filter(|p| {
                let ex = p.extension().unwrap_or_default();
                ex.eq("c") || ex.eq("h")
            })
            .collect::<Vec<_>>();

        cfiles
    }
}

fn find_orbit2_includes() -> Vec<PathBuf> {
    pkg_config::Config::new()
        .atleast_version("2.14.19")
        .print_system_cflags(true)
        .probe("ORBit-2.0")
        // Expect is fine as this has orbit2-sys as a dependency
        .expect("Cannot find ORBit-2.0 with pkg_config")
        .include_paths
    //.iter()
    //.map(|p| format!("-I{}", p.display()))
    //.collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use tempdir::TempDir;

    use super::*;
    #[test]
    fn base() {
        assert_eq!(CommonBuilder::new().service_name, "");
        assert_eq!(CommonBuilder::new().orbit_idl, "orbit-idl-2");
    }

    #[test]
    fn service_name() {
        assert_eq!(
            CommonBuilder::default()
                .service_name("foo".into())
                .service_name,
            "foo"
        );
    }

    fn test_fixture() -> (PathBuf, PathBuf) {
        let tmp_path = TempDir::new("example")
            .expect("Cannot create temp dir")
            .into_path();
        let idl_path = tmp_path.join("echo.idl");
        fs::write(
            idl_path.clone(),
            "interface Echo {
    void echoString(in string input);
};",
        )
        .expect("can write example");
        (tmp_path, idl_path)
    }

    // see https://doc.rust-lang.org/reference/conditional-compilation.html
    //#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
    #[test]
    fn test_generate() {
        let (tmp_path, idl_path) = test_fixture();

        let builder = CommonBuilder::new()
            .idl_file(idl_path.clone())
            .out_path(tmp_path.clone());

        // Need to set OUT_DIR to tmp_path
        env::set_var("OUT_DIR", tmp_path);
        env::set_var("TARGET", env!("TEST_TARGET"));
        env::set_var("HOST", env!("TEST_TARGET")); // No cross compilation
        env::set_var("OPT_LEVEL", "0");

        //env::set_var("TARGET", "x86_64-unknown-linux-gnu");
        builder.generate();
    }

    #[test]
    fn generate_ccode() {
        let (tmp_path, idl_path) = test_fixture();

        let mut cfiles = CommonBuilder::new()
            .idl_file(idl_path.clone())
            .out_path(tmp_path.clone())
            .generate_common_ccode();

        //assert!( cfiles.iter().zip())
        let filenames = vec!["echo-common.c", "echo-skels.c", "echo-stubs.c", "echo.h"];
        cfiles.sort();
        //assert_eq!(cfiles, ["ba", "ba"].map(|s| PathBuf::from_str(s).unwrap()));
        assert!(cfiles
            .iter()
            .zip(filenames.iter())
            .all(|(p, &s)| p.ends_with(s)));

        assert_eq!(cfiles.len(), 4);
    }

    #[test]
    fn find_includes_test() {
        assert!(find_orbit2_includes().len() > 0);
    }
}
