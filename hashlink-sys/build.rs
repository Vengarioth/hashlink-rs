use std::env;
use std::path::PathBuf;

use bindgen;
use cmake::Config;

fn main() {

    let output_path = Config::new("../hashlink")
        .define("WITH_VIDEO", "false")
        .define("BUILD_TESTING", "false")
        .build();

    let mut output_lib_path = output_path.clone();
    output_lib_path.push("lib");

    println!("cargo:rustc-link-search=native={}", output_lib_path.display());
    println!("cargo:rustc-link-lib=static=libhl");
    println!("cargo:include={}", output_path.display());

    let mut output_include_path = output_path.clone();
    output_include_path.push("include");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("--include-directory=../hashlink/src/")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
