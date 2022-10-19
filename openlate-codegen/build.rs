// build.rs

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use openlate_template;

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("nidaqmx.rs");
  let mut f = File::create(&dest_path).unwrap();

  let rendered: String = openlate_template::nidaqmx::render().unwrap();
  f.write_all(rendered.as_bytes()).unwrap();
}
