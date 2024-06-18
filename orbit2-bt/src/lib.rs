use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct Builder {
    service_name: String,
    out_path: PathBuf,
    idl_file: String,
    orbit_idl: String,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            orbit_idl: "orbit-idl-2".to_owned(),
            ..Default::default()
        }
    }

    pub fn service_name(self, service_name: String) -> Self {
        Builder {
            service_name,
            ..self
        }
    }

    pub fn idl_file(self, idl_file: String) -> Self {
        Builder { idl_file, ..self }
    }

    pub fn out_path(self, out_path: PathBuf) -> Self {
        Builder { out_path, ..self }
    }

    fn generate_ccode(self) {
        use std::process::Command;
        let _ = Command::new(self.orbit_idl)
            .arg(format!(
                "--output-dir={}",
                self.out_path.to_str().expect("Not unicode dirname")
            ))
            .arg(self.idl_file)
            .output()
            .expect("Failed to run IDL");
    }
}

fn find_include_options() -> Vec<String> {
    pkg_config::Config::new()
        .atleast_version("2.14.19")
        .print_system_cflags(true)
        .probe("ORBit-2.0")
        // Expect is fine as this has orbit2-sys as a dependency
        .expect("Cannot find ORBit-2.0 with pkg_config")
        .include_paths
        .iter()
        .map(|p| format!("-I{}", p.display()))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempdir::TempDir;

    use super::*;
    #[test]
    fn base() {
        assert_eq!(Builder::new().service_name, "");
        assert_eq!(Builder::new().orbit_idl, "orbit-idl-2");
    }

    #[test]
    fn servive_name() {
        assert_eq!(
            Builder::default().service_name("foo".into()).service_name,
            "foo"
        );
    }

    #[test]
    fn generate_ccode() {
        let tmp_dir = TempDir::new("example").expect("Cannot create temp dir");
        let idl_path = tmp_dir.into_path().join("echo.idl");
        fs::write(
            idl_path,
            "interface Echo {
    void echoString(in string input);
};",
        )
        .expect("can write example");

        Builder::new()
            .idl_file("tests/echo.idl".into())
            .out_path(PathBuf::from("tests"))
            .generate_ccode();
    }

    #[test]
    fn find_includes() {
        assert!(find_include_options().len() > 0);
    }
}
