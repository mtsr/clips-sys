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

  let _ = Command::new("make").current_dir("CLIPS/core").status();

  println!("cargo:rustc-link-lib=static=clips");
  println!("cargo:rustc-link-search=./CLIPS/core/");

  // The bindgen::Builder is the main entry point
  // to bindgen, and lets you build up options for
  // the resulting bindings.
  let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}
