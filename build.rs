extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  
  // clone or pull
  if !out_dir.join("CLIPS/.git").exists() {
    let _ = Command::new("git")
      .current_dir(&out_dir)
      .args(&["clone", "https://github.com/mtsr/CLIPS.git"])
      .status();
  } else {
    let _ = Command::new("git")
      .current_dir(&out_dir.join("CLIPS"))
      .args(&["pull"])
      .status();
  }

  // checkout the right branch
  let _ = Command::new("git")
    .current_dir(&out_dir.join("CLIPS"))
    .args(&["checkout", "64x"])
    .status();

  // prepare make command
  let mut cmd = Command::new("make");

  // Conditionally make debug build
  if env::var("DEBUG").is_ok() {
    cmd.arg("debug");
  }
  
  // and run it
  let _ = cmd
  .current_dir(&out_dir.join("CLIPS/core"))
  .status();

  // copy wrapper
  let _ = Command::new("cp")
    .args(&["wrapper.h", &format!("{}", out_dir.display())])
    .status();

  println!("cargo:rustc-link-lib=static=clips");
  println!("cargo:rustc-link-search={}", &out_dir.join("CLIPS/core").display());

  let out_path = out_dir.join("bindings.rs");

  if !out_path.exists() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
          // The input header we would like to generate
          // bindings for.
          // use the copy in out_dir
          .header(format!("{}", out_dir.join("wrapper.h").display()))
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
