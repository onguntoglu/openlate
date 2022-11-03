use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

include!("src/mod.rs");

fn main() -> Result<(), Box<dyn Error>> {
  println!("cargo:rerun-if-changed=src/nidaqmx.rs");

  let out_dir = PathBuf::new().join(std::env::var("OUT_DIR").unwrap());
  let mut nidaqmx_out = File::create(out_dir.join("nidaqmx_gen.rs")).unwrap();

  let rendered = nidaqmx::NidaqmxGen::new();

  nidaqmx_out
    .write_all(rendered.generate().as_bytes())
    .unwrap();

  Ok(())
}
