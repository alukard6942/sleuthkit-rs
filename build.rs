extern crate bindgen;

// use std::env;
use std::path::PathBuf;

fn main() {
    // shared library.
    println!("cargo:rustc-link-lib=tsk");

    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("src/bindings");

    let bindings = bindgen::Builder::default()
        .rustified_enum("TSK_IMG_TYPE_ENUM")
        .rustified_enum("TSK_VS_TYPE_ENUM")
        .rustified_enum("TSK_FS_TYPE_ENUM")
        .header("wrapper.h")
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_comments(true)
        .clang_args([
            "-fretain-comments-from-system-headers",
            "-fparse-all-comments",
        ])
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("binds.rs"))
        .expect("Couldn't write bindings!");
}
