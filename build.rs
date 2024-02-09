extern crate bindgen;
use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-lib=raylib");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = "src/bindings.rs";

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
