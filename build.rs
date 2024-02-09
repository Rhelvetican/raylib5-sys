extern crate bindgen;
use std::{env, path::PathBuf};

use bindgen::CargoCallbacks;

fn main() {
    println!("cargo:rustc-link-lib=raylib");
    println!("cargo:rerun-if-changed=wrapper.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    if std::env::var("DOCS_RS").is_ok() {
        std::fs::copy("bindings-for-docs-rs.rs", out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    } else {
        let bindings = bindgen::Builder::default()
            .header("wrapper.h")
            .parse_callbacks(Box::new(CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
