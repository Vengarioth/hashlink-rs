use std::env;
use std::path::PathBuf;

use bindgen;
use cmake::Config;

fn build_libhl_full()
{
    let mut build = cc::Build::new();

    // Global config
    build.cpp        (true);
    build.static_flag(true);
    build.out_dir    ("_deploy");

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
    println!("cargo:rustc-link-search=./_deploy");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("--include-directory=../vendor/hashlink/src")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
