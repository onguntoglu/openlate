pub mod nidaqmx;

use serde::{Deserialize, Serialize};
use std::error::Error;

use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn main() -> Result<(), Box<dyn Error>> {
  let nidaqmx = nidaqmx::NidaqmxMetadata::default();
  // let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new("nidaqmx_gen.rs");
  let mut f = File::create(dest_path).unwrap();

  // f.write_all(rendered.as_bytes()).unwrap();

  Ok(())
}
