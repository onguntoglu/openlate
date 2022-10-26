use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

include!("src/lib.rs");

fn main() -> Result<(), Box<dyn Error>> {
  let out_dir = PathBuf::new()
    .join(std::env::var("OUT_DIR").unwrap())
    .join("nidaqmx_gen.rs");
  let dest_path = Path::new("nidaqmx_gen.rs");
  let mut f = File::create(dest_path).unwrap();
  let mut nidaqmx_out = File::create(out_dir).unwrap();

  let rendered = nidaqmx::NidaqmxGen::new();

  f.write_all(rendered.generate().as_bytes()).unwrap();
  nidaqmx_out
    .write_all(rendered.generate().as_bytes())
    .unwrap();

  Ok(())
}
