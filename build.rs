extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let clips_core_dir = PathBuf::from("CLIPS/core");

  let files = 
      std::fs::read_dir(&clips_core_dir)
      .expect("CLIPS core dir not found")
      .map(|entry| entry.expect("not a valid entry"))
      .filter(|entry| {
        !entry.file_type().expect("not a valid filetype").is_dir() 
      })
      .map(|entry| PathBuf::from(entry.path()))
      .filter(|path| match path.extension() {
        None => false,
        Some(extension) => extension == "c",
      })
      // TODO #define DEVELOPER 1 if feature is set
      .filter(|path| cfg!(feature = "developer") || !path.ends_with("developr.c"));

  eprintln!("{:?}", files);

  cc::Build::new()
    .files(files)
    .include(&clips_core_dir)
    // .warnings(false)
    .out_dir(&out_dir)
    // Exclude current CLIPS compile warnings
    .flag("-Wno-unused-parameter")
    .flag("-Wno-missing-field-initializers")
    .compile("clips");
  
  let _res = std::fs::copy(PathBuf::from("wrapper.h"), &out_dir.join("wrapper.h"));

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
          .header("CLIPS/core/clips.h")
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
