use openlate_sys;
use serde::{self, Deserialize, Serialize};
use serde_json::{Number, Result, Value};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct Integer(String);

impl From<Integer> for i32 {
  fn from(attr_val: Integer) -> Self {
    match attr_val {
      x => x.0.parse().unwrap(),
    }
  }
}

type Attr = HashMap<Integer, Fields>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attributes {
  buffer: Attr,
  calibration_info: Attr,
  channel: Attr,
  device: Attr,
  export_signal: Attr,
  persisted_channel: Attr,
  persisted_scale: Attr,
  persisted_task: Attr,
  physical_channel: Attr,
  read: Attr,
  real_time: Attr,
  scale: Attr,
  system: Attr,
  task: Attr,
  trigger: Attr,
  watchdog: Attr,
  write: Attr,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fields {
  access: String,
  name: String,
  resettable: bool,
  r#type: String,
}

#[cfg(test)]
fn serde_nidaqmx() -> Result<Attributes> {
  let file = File::open("metadata/nidaqmx/attributes.json").unwrap();
  let reader = BufReader::new(file);

  // Read the JSON contents of the file as an instance of `User`.
  let u = serde_json::from_reader(reader).unwrap();

  // Return the `User`.
  Ok(u)
}

#[test]
fn test_json_to_nidaqmx() {
  let u = serde_nidaqmx().unwrap();
  // println!("{:#?}", u);
  println!("{:#?}", u);
}
