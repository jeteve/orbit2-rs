use std::{fs, path::PathBuf, process::Output};

use bindgen::BindgenError;

#[derive(Debug)]
pub enum Error {
    Bingen(BindgenError),
    Io(std::io::Error),
    NoHeaderFound,
    BadPathBuf(PathBuf),
    CommandFailure(Output),
}

impl From<BindgenError> for Error {
    fn from(value: BindgenError) -> Self {
        Error::Bingen(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

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

    /// Note this can panic! as it uses [`cc::Build`]
    pub fn generate(&self) -> Result<()> {
        let cfiles = self.generate_common_ccode()?;

        let mut cc = cc::Build::new();
        let cc = cfiles.iter().fold(&mut cc, |cc, f| cc.file(f));

        cc.include(self.out_path.clone())
            .includes(self.includes.clone())
            .compile(&format!("{}_common", self.service_name));

        // Time to do some bindgen stuff.
        let the_header = cfiles
            .iter()
            .filter(|f| f.extension().unwrap_or_default().eq("h"))
            .take(1)
            .next()
            .ok_or(Error::NoHeaderFound)?;

        let _bindgen = bindgen::Builder::default()
            .header(
                the_header
                    .to_str()
                    .ok_or(Error::BadPathBuf(the_header.clone()))?
                    .to_string(),
            )
            .allowlist_recursively(false)
            .clang_args(self.includes.iter().map(|p| format!("-I{}", p.display())))
            .allowlist_item("POA_Echo.*")
            .generate()?;

        Ok(())
    }

    fn generate_common_ccode(&self) -> Result<Vec<PathBuf>> {
        use std::process::Command;
        let output = Command::new(self.orbit_idl.clone())
            .arg(format!(
                "--output-dir={}",
                self.out_path.clone().to_str().expect("Not unicode dirname")
            ))
            .arg(self.idl_file.clone())
            .output()?;

        if !output.status.success() {
            return Err(Error::CommandFailure(output));
        }

        let cfiles: Result<_> = fs::read_dir(self.out_path.clone())?
            .map(|entry: std::result::Result<fs::DirEntry, std::io::Error>| entry.map(|d| d.path()))
            .filter(|p| {
                if p.is_ok() {
                    let p = p.as_ref().unwrap();
                    let ex = p.extension().unwrap_or_default();
                    ex.eq("c") || ex.eq("h")
                } else {
                    false
                }
            })
            .map(|r| r.map_err(|e| Error::Io(e)))
            .collect::<Result<Vec<PathBuf>>>();

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
        assert!(builder.generate().is_ok());
    }

    #[test]
    fn generate_ccode() {
        let (tmp_path, idl_path) = test_fixture();

        let cfiles = CommonBuilder::new()
            .idl_file(idl_path.clone())
            .out_path(tmp_path.clone())
            .generate_common_ccode();
        assert!(cfiles.is_ok());
        let mut cfiles = cfiles.unwrap();

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
