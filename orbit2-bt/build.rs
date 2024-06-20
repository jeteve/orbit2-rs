fn main() {
    println!(
        "cargo:rustc-env=TEST_TARGET={}",
        std::env::var("TARGET").unwrap()
    );
}
