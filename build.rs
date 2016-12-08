#[cfg(feature = "serde_codegen")]
extern crate serde_codegen;

#[cfg(feature = "serde_codegen")]
fn main() {
    use std::env;
    use std::path::Path;
    let out_dir = env::var_os("OUT_DIR").unwrap();

    serde_codegen::expand("src/hue.in.rs", Path::new(&out_dir).join("hue.rs")).unwrap();
    serde_codegen::expand("src/json.in.rs", Path::new(&out_dir).join("json.rs")).unwrap();
}

#[cfg(not(feature = "serde_codegen"))]
fn main() {}
