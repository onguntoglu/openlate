use bindgen::{self, CodegenConfig, EnumVariation};

use std::env;
use std::path::{Path, PathBuf};

fn main() {
  // Tell cargo to look for shared libraries in the specified directory
  println!("cargo:rustc-link-search=/lib/x86_64-linux-gnu");

  // Tell cargo to tell rustc to link the system nidaqmx
  // shared library.
  println!("cargo:rustc-link-lib=nidaqmx");

  // Tell cargo to invalidate the built crate whenever the wrapper changes
  println!("cargo:rerun-if-changed=../includes/wrapper.h");

  let mut args = Vec::new();
  args.push("-fparse-all-comments");
  args.push("-fretain-comments-from-system-headers");

  // The bindgen::Builder is the main entry point
  // to bindgen, and lets you build up options for
  // the resulting bindings.
  let bindings = bindgen::Builder::default()
    // The input header we would like to generate
    // bindings for.
    .header("../includes/wrapper.h")
    .rustfmt_bindings(true)
    .clang_args(args.into_iter())
    // .generate_block(true)
    .ctypes_prefix("cty")
    // .new_type_alias_deref("TaskHandle")
    // .new_type_alias_deref("CalHandle")
    .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
    .default_enum_style(EnumVariation::Rust {
      non_exhaustive: true,
    })
    .rustified_non_exhaustive_enum("DaqmxStat")
    // .opaque_type("TaskHandle")
    // .opaque_type("CalHandle")
    // .array_pointers_in_arguments(true)
    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed.
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
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
