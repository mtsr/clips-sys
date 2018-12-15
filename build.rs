extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
  if !Path::new("CLIPS/.git").exists() {
    let _ = Command::new("git")
      .args(&["submodule", "update", "--init"])
      .status();
  }

  if !Path::new("CLIPS/core/clips").exists() {
    let _ = Command::new("make")
      // .arg("debug")
      .current_dir("CLIPS/core")
      .status();
  }

  println!("cargo:rustc-link-lib=static=clips");
  println!("cargo:rustc-link-search=./CLIPS/core/");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

  if !out_path.exists() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
          // The input header we would like to generate
          // bindings for.
          .header("wrapper.h")
          .derive_debug(true)
          .impl_debug(true)
          .derive_default(true)
          // Finish the builder and generate the bindings.
          .generate()
          // Unwrap the Result and panic on failure.
          .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
      .write_to_file(out_path)
      .expect("Couldn't write bindings!");
  }
}
