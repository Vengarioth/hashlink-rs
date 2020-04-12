use std::env;
use std::path::PathBuf;

use bindgen;
use cmake::Config;

fn build_libhl_full()
{
    let mut out_dir = PathBuf::from(env::var("out_dir").unwrap());
    out_dir.push("_deploy");

    let mut build = cc::Build::new();

    // Global config
    build.cpp(true);
    build.static_flag(true);
    build.out_dir(out_dir);

    // C-Flags    
    build.flag("-std=c++11");

    // -I
    build.include("../vendor/hashlink/src");
    build.include("../vendor/hashlink/include/pcre");

    let std_src = [
          "wrapper.c"
    ];

    for source_file in &std_src {
        build.file(&source_file);
    }

    // Linker configuration 
    build.static_crt(true);

    // -L
    // -l
    
    // GO
    build.compile("hlfull");
}

fn main() {
    build_libhl_full();

    let mut out_dir = PathBuf::from(env::var("out_dir").unwrap());
    out_dir.push("_deploy");

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=hlfull");
    println!("cargo:rustc-link-lib=dylib=ws2_32");
    println!("cargo:rustc-link-lib=dylib=user32");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("--include-directory=../vendor/hashlink/src")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let mut bindings_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    bindings_path.push("src");

    bindings
        .write_to_file(bindings_path.join("ffi.rs"))
        .expect("Couldn't write bindings!");
}
