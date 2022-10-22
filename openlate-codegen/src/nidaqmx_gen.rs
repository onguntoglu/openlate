use lazy_static::lazy_static;
use regex::Regex;
use serde::{self, Deserialize, Serialize};
use serde_json::{self, Result};
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

lazy_static! {
  static ref NUMBER_AT_START: Regex = Regex::new(r"^(\d+)").unwrap();
}

#[derive(Serialize, Debug)]
pub struct NidaqmxMetadata {
  pub enums: EnumMetadata,
  pub func: FuncMetadata,
  pub attr: AttrMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuncMetadata(HashMap<FuncName, FuncFields>);

#[derive(Debug, Serialize, Deserialize)]
pub struct FuncFields {
  parameters: Vec<FuncParameter>,
  returns: FuncReturn,
}

#[derive(Debug, Serialize, Eq, PartialEq, Hash, Deserialize)]
pub struct FuncName(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct FuncReturn(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct FuncParameter {
  direction: String,
  name: String,
  r#type: Option<String>,
  size: Option<ParameterSize>,
  r#enum: Option<EnumName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterSize {
  mechanism: String,
  value: String,
}

#[derive(Hash, Default, Eq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct EnumName(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumValues {
  values: Vec<EnumVariant>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumDescription {
  description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumVariant {
  documentation: Option<EnumDescription>,
  name: String,
  value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnumMetadata(HashMap<EnumName, EnumValues>);

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Integer(String);

impl From<Integer> for i32 {
  fn from(attr_val: Integer) -> Self {
    match attr_val {
      x => x.0.parse().unwrap(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttrContent(HashMap<Integer, AttrProperty>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttrMetadata(HashMap<String, AttrContent>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttrProperty {
  access: String,
  name: String,
  resettable: bool,
  r#type: String,
}

impl Default for NidaqmxMetadata {
  fn default() -> NidaqmxMetadata {
    let metadata_dir = PathBuf::from("metadata/nidaqmx/");
    let metadata = ["functions.json", "attributes.json", "enums.json"];
    let mut map: HashMap<String, BufReader<File>> = HashMap::new();
    for meta in metadata {
      let m =
        metadata_dir.join(PathBuf::from(meta).as_os_str().to_str().unwrap());
      println!("{:#?}", m);
      let file = File::open(m).unwrap();
      map.insert(meta.replace(".json", "").to_string(), BufReader::new(file));
    }

    let enums: EnumMetadata =
      serde_json::from_reader(map.get_mut("enums").unwrap()).unwrap();
    let func: FuncMetadata =
      serde_json::from_reader(map.get_mut("functions").unwrap()).unwrap();
    let attr: AttrMetadata =
      serde_json::from_reader(map.get_mut("attributes").unwrap()).unwrap();

    NidaqmxMetadata { enums, func, attr }
  }
}

impl SwapNumberToBack for EnumVariant {
  fn swap_number(mut self) -> EnumVariant {
    let foo = match self.name.split("_").next() {
      None => self.name,
      Some(num) => match num.as_bytes()[0].is_ascii_digit() {
        true => {
          self
            .name
            .strip_prefix(num)
            .expect("Could not strip prefix")
            .strip_prefix("_")
            .unwrap_or_default()
            .to_owned()
            + "_"
            + num
        }
        false => self.name,
      },
    };
    self.name = foo;
    self
  }
}

pub trait SwapNumberToBack {
  fn swap_number(self) -> Self;
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::{self, Result};
  use std::{fs::File, io::BufReader};

  fn serde_func() -> Result<FuncMetadata> {
    let file = File::open("metadata/nidaqmx/functions.json").unwrap();
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader).unwrap();
    Ok(u)
  }

  #[cfg(test)]
  fn serde_attr() -> Result<AttrMetadata> {
    let file = File::open("metadata/nidaqmx/attributes.json").unwrap();
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader).unwrap();
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
    println!("{:#?}", u);
  }

  #[test]
  fn test_enum() {
    let u = serde_enum().unwrap();
    println!("{:#?}", u);
  }

  #[test]
  fn test_swap_strnum_toback() {
    let variant = EnumVariant {
      documentation: Some(EnumDescription {
        description: String::from("4-wire"),
      }),
      name: String::from("4_WIRE"),
      value: 4,
    };
    println!("{:#?}", variant.swap_number());
  }
}
