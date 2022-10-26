use codegen;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{self, Deserialize, Serialize};
use serde_json::{self};
use std::{
  collections::HashMap, fs::File, io::BufReader, ops::Deref, path::PathBuf,
};

lazy_static! {
  static ref NUMBER_AT_START: Regex = Regex::new(r"^(\d+)").unwrap();
}

pub struct NidaqmxGen {
  scope: codegen::Scope,
  nidaqmx: NidaqmxMetadata,
}

impl NidaqmxGen {
  pub fn new() -> NidaqmxGen {
    NidaqmxGen {
      scope: codegen::Scope::new(),
      nidaqmx: NidaqmxMetadata::default(),
    }
    .generate_enums()
  }
  fn generate_impl_from(
    &self,
    generic: &str,
    target: &str,
    block: codegen::Block,
  ) -> codegen::Impl {
    let mut impl_from = codegen::Impl::new(codegen::Type::new(format!(
      "From<{}> for {}",
      generic, target
    )));
    let fn_from = codegen::Function::new("from")
      .arg("val", codegen::Type::new(generic))
      .ret(codegen::Type::new(target))
      .push_block(block)
      .to_owned();
    return impl_from.push_fn(fn_from).to_owned();
  }

  fn generate_enums(mut self) -> NidaqmxGen {
    for enm in self.nidaqmx.enums.iter() {
      let (enum_name, enm_values) = (enm.0.to_string(), enm.1);
      let mut curr_enm = codegen::Enum::new(enum_name.to_owned());
      let mut block_raw = codegen::Block::new("match val");
      let mut block_enum = codegen::Block::new("match val");

      for variant in enm_values.iter() {
        let fixed_var = variant.clone().prefix_letter();
        let mut var = codegen::Variant::new(fixed_var.to_owned().name);
        match fixed_var.documentation {
          Some(doc) => var.annotation(doc.add_quotes().description),
          None => &mut var,
        };
        curr_enm.push_variant(var);
        block_raw.line(format!(
          "{}_i32 => {}::{},",
          fixed_var.value, enum_name, fixed_var.name
        ));
        block_enum.line(format!(
          "{}::{} => {}_i32,",
          enum_name, fixed_var.name, fixed_var.value
        ));
      }
      let fr_raw = self.generate_impl_from("i32", &enum_name, block_raw);
      let fr_enum = self.generate_impl_from(&enum_name, "i32", block_enum);
      self
        .scope
        .push_enum(curr_enm.vis("pub").to_owned())
        .push_impl(fr_raw.to_owned())
        .push_impl(fr_enum.to_owned());
    }
    self
  }

  pub fn generate(&self) -> String {
    self.scope.to_string()
  }
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

#[derive(Hash, Default, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct EnumName(String);

impl Deref for EnumMetadata {
  type Target = HashMap<EnumName, EnumValues>;

  fn deref(&self) -> &Self::Target {
    &self.0 //pointer to Inner value
  }
}

impl Deref for EnumName {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0 //pointer to Inner value
  }
}

impl Deref for EnumValues {
  type Target = Vec<EnumVariant>;
  fn deref(&self) -> &Self::Target {
    &self.values //pointer to Inner value
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumValues {
  values: Vec<EnumVariant>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumDescription {
  description: String,
}

impl EnumDescription {
  fn add_quotes(mut self) -> Self {
    let front = &"#[doc=\"".to_owned();
    self.description = front.to_owned() + &self.description + &"\"]".to_owned();
    self
  }
}

impl Deref for EnumDescription {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.description //pointer to Inner value
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl EnumProcessor for EnumVariant {
  fn prefix_letter(mut self) -> Self {
    let foo = match self.name.as_bytes()[0].is_ascii_digit() {
      true => "d".to_owned() + &self.name,
      false => self.name,
    };
    self.name = foo;
    self
  }
  fn swap_number(mut self) -> Self {
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

pub trait EnumProcessor {
  fn swap_number(self) -> Self;
  fn prefix_letter(self) -> Self;
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
  #[test]
  fn test_gen_enums() {
    let gen = NidaqmxGen::new();
    let rendered = gen.generate_enums().generate();
    println!("{:#?}", rendered);
  }
}
