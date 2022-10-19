pub mod nidaqmx;
use nidaqmx::metadata::NidaqmxMetadata;
use serde::Serialize;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tera::Context;
use tera::Tera;

#[derive(Serialize, Debug)]
struct Nidaqmx {
  nidaqmx: NidaqmxMetadata,
}

pub fn main() -> Result<(), Box<dyn Error>> {
  // Use globbing
  let tera = match Tera::new("src/nidaqmx/templates/**/*.tera") {
    Ok(t) => t,
    Err(e) => {
      println!("Parsing error(s): {}", e);
      ::std::process::exit(1);
    }
  };
  // Using the tera Context struct
  let mut context = Context::new();
  let daqmx = NidaqmxMetadata::default();
  context.insert("nidaqmx", &daqmx);
  let rendered = tera.render("values.rs.tera", &context)?;

  // let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new("nidaqmx_gen.rs");
  let mut f = File::create(dest_path).unwrap();

  f.write_all(rendered.as_bytes()).unwrap();

  Ok(())
}
