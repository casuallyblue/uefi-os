use std::env;

use std::path::Path;
use std::path::PathBuf;

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target/x86_64-unknown-uefi")
        .join(build_type);
    path
}

fn main() {
    println!("cargo:rerun-if-changed=boot/startup.nsh");
    let startup_nsh_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("boot")
        .join("startup.nsh");
    let output_path = get_output_path();
    let startup_nsh_output_path = Path::new(&output_path).join("startup.nsh");

    let res = std::fs::copy(startup_nsh_path, startup_nsh_output_path);
    println!("cargo::warning={:#?}", res);
}
