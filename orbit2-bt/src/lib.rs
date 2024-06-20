use std::{fs, path::PathBuf, process::Output};

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

    fn generate_ccode(self) -> Vec<PathBuf> {
        use std::process::Command;
        let output = Command::new(self.orbit_idl)
            .arg(format!(
                "--output-dir={}",
                self.out_path.clone().to_str().expect("Not unicode dirname")
            ))
            .arg(self.idl_file)
            .output()
            .expect("Failed to run IDL");
        assert!(output.status.success());
        let cfiles = fs::read_dir(self.out_path.clone())
            .expect("Can list tmp_path")
            .map(|r| r.expect("Good dir entry"))
            .map(|d| d.path())
            .filter(|p| {
                let ex = p.extension().unwrap_or_default();
                ex.eq("c")
            })
            .collect::<Vec<_>>();

        cfiles
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
    use std::{fs, str::FromStr};

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

        let cfiles = Builder::new()
            .idl_file(idl_path.to_str().map(|s| String::from(s)).unwrap())
            .out_path(tmp_path.clone())
            .generate_ccode();
        //assert_eq!(cfiles, ["ba", "ba"].map(|s| PathBuf::from_str(s).unwrap()));
        assert_eq!(cfiles.len(), 3);
    }

    #[test]
    fn find_includes() {
        assert!(find_include_options().len() > 0);
    }
}
