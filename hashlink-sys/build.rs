use std::env;
use std::path::PathBuf;

use bindgen;
use cmake::Config;

fn main() {

    // TODO make these flags cargo features and document how to add
    // dependencies (e.g. see hashlink docs)
    let output_path = Config::new("../vendor/hashlink")
        .define("WITH_DIRECTX", "false")
        .define("WITH_FMT", "false")
        .define("WITH_OPENAL", "false")
        .define("WITH_SDL", "false")
        .define("WITH_SQLITE", "false")
        .define("WITH_SSL", "false")
        .define("WITH_UI", "false")
        .define("WITH_UV", "false")
        .define("WITH_VIDEO", "false")
        .define("BUILD_TESTING", "false")
        .build();

    let mut output_lib_path = output_path.clone();
    output_lib_path.push("lib");

    let mut output_bin_path = output_path.clone();
    output_bin_path.push("bin");

    println!("cargo:rustc-link-search=native={}", output_lib_path.display());
    println!("cargo:rustc-link-search=native={}", output_bin_path.display());
    
    println!("cargo:rustc-link-lib=static=libhl");
    println!("cargo:rustc-link-lib=dylib=libhl");
    println!("cargo:rustc-link-lib=dylib=ws2_32");
    println!("cargo:rustc-link-lib=dylib=user32");

    let mut output_include_path = output_path.clone();
    output_include_path.push("include");
    println!("cargo:include={}", output_include_path.display());

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("--include-directory={}", output_include_path.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
