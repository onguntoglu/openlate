use openlate_sys;
use serde::{self, Deserialize, Serialize};
use serde_json::{Number, Result, Value};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct EnumName(String);

#[derive(Debug, Deserialize)]
struct EnumValues {
  values: Vec<EnumFields>,
}

#[derive(Debug, Default, Deserialize)]
struct EnumDescription {
  description: String,
}

#[derive(Debug, Deserialize)]
struct EnumFields {
  #[serde(default)]
  documentation: EnumDescription,
  name: String,
  value: i32,
}

fn default_resource() -> String {
  "".to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct EnumMetadata(HashMap<EnumName, EnumValues>);

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
fn serde_attr() -> Result<Attributes> {
  let file = File::open("metadata/nidaqmx/attributes.json").unwrap();
  let reader = BufReader::new(file);

  // Read the JSON contents of the file as an instance of `User`.
  let u = serde_json::from_reader(reader).unwrap();

  // Return the `User`.
  Ok(u)
}

#[cfg(test)]
fn serde_enum() -> Result<EnumMetadata> {
  let file = File::open("metadata/nidaqmx/enums.json").unwrap();
  let reader = BufReader::new(file);
  let u = serde_json::from_reader(reader).unwrap();

  Ok(u)
}

#[test]
fn test_attr() {
  let u = serde_attr().unwrap();
  // println!("{:#?}", u);
  println!("{:#?}", u);
}
#[test]
fn test_enum() {
  let u = serde_enum().unwrap();
  // println!("{:#?}", u);
  println!("{:#?}", u);
}
