pub mod nidaqmx;

use serde::{Deserialize, Serialize};
use std::error::Error;

use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn main() -> Result<(), Box<dyn Error>> {
  // let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new("nidaqmx_gen.rs");
  let mut f = File::create(dest_path).unwrap();

  let rendered = nidaqmx::NidaqmxGen::new();

  f.write_all(rendered.generate().as_bytes()).unwrap();

  Ok(())
}
