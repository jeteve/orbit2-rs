# How to build a common IDL lib (no service code, just types and client)

For the rest of this readme, we assume that your library containing your IDLs binding
is called `$PROJECT_IDLS_NAME`. We'll also work from an example `echo.idl` to illustrate how to build
the binding.

## Make a new lib

```sh
cargo new --lib $PROJECT_IDLS_NAME
cd $PROJECT_IDLS_NAME
```

## Depends on this, in your `Cargo.toml`, as well as orbit2-sys as a standard dependency

```cargo
[dependencies]
orbit2-sys = ">=0.0.1"

[build-dependencies]
orbit2-buildtools = ">=0.0.1"
```

## Add your idls in 'static'

```sh
$ mkdir static/
# And put your IDL(s?), for instance:
$ cat static/echo.idl 
interface Echo {
    void echoString(in string input);
};
```

## Make a `build.rs` to compile the idl

Example `build.rs` (assuming echo.idl)

```rust
use std::path::PathBuf;

use orbit2_buildtools::CommonBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let idl_path = PathBuf::from("static/echo.idl");
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // generate the C code plus the Rust binding
    let r = CommonBuilder::new("Echo")
        .idl_file(&idl_path)
        .out_path(&out_path)
        .generate()?;

    // make the binding path available in the rest of the compilation
    println!(
        "cargo:rustc-env=ECHO_IDL_BINDING={:?}",
        r.binding_file.as_path()
    );

    Ok(())
}
```

## Structure the generated code in your lib.rs (or a submodule if you wish too)

```rust

use orbit2_sys::core::*;
include!(env!("ECHO_IDL_BINDING"));

```

And that's it. Now you can use your module in your client and server applications.
