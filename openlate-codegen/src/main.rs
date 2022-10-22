pub mod nidaqmx_gen;

use nidaqmx_gen::EnumVariant;
use nidaqmx_gen::NidaqmxMetadata;
use nidaqmx_gen::SwapNumberToBack;
use serde::Serialize;
use std::error::Error;

use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Debug)]
struct Nidaqmx {
  nidaqmx: nidaqmx_gen::NidaqmxMetadata,
}

pub fn main() -> Result<(), Box<dyn Error>> {
  // Use globbing
  let tera = match Tera::new("src/templates/**/*.tera") {
    Ok(t) => t,
    Err(e) => {
      println!("Parsing error(s): {}", e);
      ::std::process::exit(1);
    }
  };

  // Using the tera Context struct
  let mut context = Context::new();
  let daqmx = nidaqmx_gen::NidaqmxMetadata::default();
  context.insert("nidaqmx", &daqmx);
  let rendered = tera.render("nidaqmx_gen.rs.tera", &context)?;

  // let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new("nidaqmx_gen.rs");
  let mut f = File::create(dest_path).unwrap();

  f.write_all(rendered.as_bytes()).unwrap();

  Ok(())
}
