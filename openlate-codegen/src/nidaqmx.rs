use codegen as cgen;
use inflector::Inflector;
use serde::{self, Deserialize, Serialize};
use serde_json;
use std::{
  collections::HashMap,
  fs::File,
  io::BufReader,
  ops::{Deref, DerefMut},
  path::PathBuf,
};

fn typedef(r#type: impl Into<String>) -> cgen::Type {
  cgen::Type::new(r#type)
}

fn gen_attr_with_payload(
  attr: (AttrName, AttrContent),
) -> (cgen::Struct, cgen::Impl) {
  let (attr_name, attr_values) = (
    attr.0.clone().to_string().to_pascal_case().to_owned(),
    attr.1.clone(),
  );
  let mut curr_attr = cgen::Struct::new(&attr_name)
    .attr("non_exhaustive")
    .to_owned();
  let mut impl_attr = cgen::Impl::new(typedef(attr_name));
  for content in attr_values.0.iter() {
    let (attr_value, attr_property) = content;
    let attr_variant = attr_property.to_owned().prefix_letter("X").name;
    // .to_pascal_case();
    impl_attr.associate_const(
      attr_variant,
      "DaqmxAttr",
      format!(
        "DaqmxAttr {{ value: {}, access: {}, resettable: {}}}",
        attr_value.0,
        String::from(AttrAccessEnum::from(attr_property.access.0.to_owned()))
          .to_owned(),
        attr_property.resettable.to_owned(),
      )
      .to_owned(),
      "pub",
    );
  }
  return (curr_attr, impl_attr);
}

fn generate_impl_from(
  generic: &str,
  target: &str,
  mut block: cgen::Block,
  wildcard: bool,
) -> cgen::Impl {
  match wildcard {
    true => block.line("_ => panic!(\"Invalid raw integer\"),"),
    false => &mut block,
  };
  let mut impl_from =
    cgen::Impl::new(typedef(format!("From<{}> for {}", generic, target)));
  let fn_from = cgen::Function::new("from")
    .arg("val", typedef(generic))
    .ret(typedef(target))
    .push_block(block)
    .to_owned();
  return impl_from.push_fn(fn_from).to_owned();
}

pub struct NidaqmxGen {
  scope: cgen::Scope,
  nidaqmx: NidaqmxMetadata,
}

impl NidaqmxGen {
  pub fn new() -> NidaqmxGen {
    NidaqmxGen {
      scope: cgen::Scope::new(),
      nidaqmx: NidaqmxMetadata::default(),
    }
    // .generate_funcs()
    .generate_etc()
    .generate_enums()
    .generate_attrs()
    // .generate_accrs()
  }

  fn generate_etc(mut self) -> NidaqmxGen {
    self.scope.import("std::any", "Any");
    self.scope.import("openlate_sys::nidaqmx", "*");
    self
  }

  fn generate_funcs(mut self) -> NidaqmxGen {
    let metadata = self.nidaqmx.func.0.drain();
    for (name, fields) in metadata {
      let mut f = cgen::Function::new(name.0);
    }
    self
  }

  fn generate_accrs(mut self) -> NidaqmxGen {
    let accr_ns = self.nidaqmx.accr.0.drain();
    for mut acc in accr_ns {
      let (_accr_nspace, accr_mem) =
        (acc.0.to_owned().to_pascal_case(), acc.1.drain());
      for accr in accr_mem {
        for subm in accr.1 .0 {
          let mut curr_accr =
            cgen::Function::new(String::from("DAQmx") + &subm.0);
          for par in subm.1.parameter {
            match (par.direction.as_str(), par.r#enum, par.size) {
              ("in", Some(enm), None) => {
                curr_accr.arg(&par.name, enm.0);
              }
              ("in", None, None) => match par.name.as_str() {
                "bufferSize" | "arraySizeInElements" => (),
                &_ => {
                  curr_accr.arg(&par.name, typedef(par.r#type));
                }
              },
              ("out", Some(enm), None) => {
                curr_accr.ret(typedef(enm.0));
              }
              ("out", None, None) => {
                curr_accr.ret(typedef(par.r#type));
              }
              // ("in", Some(enm), Some(sz)) => {
              //   curr_accr.ret(typedef(format!("Vec<{}>", enm.0)))
              //     .line(
              //     format!("let {} = unsafe {{ {}(std::ptr::null_mut(), 0_u32) }};"
              //     ,sz.value, subm.0))
              //   .line(format!("let mut buf = vec![0_u8; {} as usize]", sz.value))
              //     .line(
              //     format!("let errcode = unsafe {{ {}(buf.as_mut_ptr(), {}) }};"
              //     ,subm.0,sz.value))
              //   ;
              // }
              // ("in", None, Some(sz)) => {
              //   curr_accr.ret(typedef(format!("Vec<{}>", par.r#type)))
              //     .line(
              //     format!("let {} = unsafe {{ {}(std::ptr::null_mut(), 0_u32) }};"
              //     ,sz.value, subm.0))
              //   .line(format!("let mut buf = vec![0_u8; {} as usize]", sz.value))
              //     .line(
              //     format!("let errcode = unsafe {{ {}(buf.as_mut_ptr() as *mut i8, {} as u32) }};"
              //     ,subm.0,sz.value))
              //   ;
              // }
              ("out", Some(enm), Some(sz)) => {
                curr_accr.ret(typedef(format!("Vec<{}>", enm.0)))
                //   .line(
                //   format!("let {} = unsafe {{ DAQmx{}(std::ptr::null_mut(), 0_u32) }};"
                //   ,sz.value, subm.0))
                // .line(format!("let mut buf = vec![0_u8; {} as usize];", sz.value))
                //   .line(
                //   format!("let errcode = unsafe {{ DAQmx{}(buf.as_mut_ptr() as *mut i8, {} as u32) }};"
                //   ,subm.0,sz.value))
                ;
              }
              ("out", None, Some(sz)) => {
                curr_accr.ret(typedef(format!("Vec<{}>", par.r#type)))
                //   .line(
                //   format!("let {} = unsafe {{ DAQmx{}(std::ptr::null_mut(), 0_u32) }};"
                //   ,sz.value, subm.0))
                // .line(format!("let mut buf = vec![0_u8; {} as usize];", sz.value))
                //   .line(
                //   format!("let errcode = unsafe {{DAQmx{}(buf.as_mut_ptr() as *mut i8, {} as u32) }};"
                //   ,subm.0,sz.value))
                ;
              }
              (&_, _, _) => panic!("Panicked!"),
            }
          }
          self.scope.push_fn(curr_accr.line("todo!();").to_owned());
        }
      }
    }
    self
  }

  fn generate_attrs(mut self) -> NidaqmxGen {
    let attr_it = self.nidaqmx.attr.0.drain();
    for attr in attr_it {
      let (attr_struct, attr_impl) = gen_attr_with_payload(attr);
      self.scope.push_struct(attr_struct).push_impl(attr_impl);
    }
    let access_enum = cgen::Enum::new("AttrAccess")
      .push_variant(cgen::Variant::new("Read"))
      .push_variant(cgen::Variant::new("Write"))
      .push_variant(cgen::Variant::new("ReadWrite"))
      .to_owned();
    let attr_struct = cgen::Struct::new("DaqmxAttr")
      .push_field(cgen::Field::new("value", typedef("i32")))
      .push_field(cgen::Field::new("access", typedef("AttrAccess")))
      .push_field(cgen::Field::new("resettable", "bool"))
      .to_owned();
    self.scope.push_enum(access_enum);
    self.scope.push_struct(attr_struct);
    self
  }

  fn generate_enums(mut self) -> NidaqmxGen {
    for enm in self.nidaqmx.enums.iter() {
      let (enum_name, enm_values) = (enm.0.to_string(), enm.1);
      let mut curr_enm = cgen::Enum::new(enum_name.to_owned());
      let mut block_raw = cgen::Block::new("match val");
      let mut block_enum = cgen::Block::new("match val");

      for variant in enm_values.iter() {
        let fixed_var = variant.clone().prefix_letter("d");
        let mut var = cgen::Variant::new(fixed_var.name.to_pascal_case());
        match fixed_var.documentation {
          Some(doc) => var.annotation(doc.add_quotes().description),
          None => &mut var,
        };
        curr_enm.push_variant(var);
        block_raw.line(format!(
          "{}_i32 => {}::{},",
          fixed_var.value,
          enum_name,
          fixed_var.name.to_pascal_case()
        ));
        block_enum.line(format!(
          "{}::{} => {}_i32,",
          enum_name,
          fixed_var.name.to_pascal_case(),
          fixed_var.value
        ));
      }
      let fr_raw = generate_impl_from("i32", &enum_name, block_raw, true);
      let fr_enum = generate_impl_from(&enum_name, "i32", block_enum, false);
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
  pub accr: AccrMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccrMetadata(HashMap<AccrNamespace, AccrMembers>);

#[derive(Debug, Serialize, Eq, PartialEq, Hash, Deserialize)]
pub struct AccrNamespace(String);

#[derive(Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct AccrMembers(HashMap<AccrGroup, AccrSubmember>);

#[derive(Debug, Serialize, Eq, PartialEq, Hash, Deserialize)]
pub struct AccrGroup(String);

#[derive(Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct AccrSubmember(HashMap<String, AccrFields>);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccrFields {
  accessor: String,
  parameter: Vec<AccrParameter>,
  returns: String,
}

#[derive(Debug, Serialize, Eq, PartialEq, Hash, Deserialize)]
pub struct AccrName(String);

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct AccrParameter {
  direction: String,
  name: String,
  r#type: String,
  size: Option<ParameterSize>,
  r#enum: Option<EnumName>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuncMetadata(HashMap<FuncName, FuncFields>);

#[derive(Debug, Serialize, Deserialize)]
pub struct FuncFields {
  cname: Option<String>,
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

#[derive(Debug, Serialize, PartialEq, Eq, Deserialize, Hash)]
pub struct ParameterSize {
  mechanism: String,
  value: String,
}

#[derive(Hash, Default, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct EnumName(String);
#[derive(Hash, Default, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct AttrName(String);

impl Deref for EnumMetadata {
  type Target = HashMap<EnumName, EnumValues>;

  fn deref(&self) -> &Self::Target {
    &self.0 //pointer to Inner value
  }
}
impl Deref for AttrMetadata {
  type Target = HashMap<AttrName, AttrContent>;

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
impl Deref for AccrNamespace {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0 //pointer to Inner value
  }
}
impl Deref for AccrMembers {
  type Target = HashMap<AccrGroup, AccrSubmember>;

  fn deref(&self) -> &Self::Target {
    &self.0 //pointer to Inner value
  }
}
impl DerefMut for AccrMembers {
  fn deref_mut(&mut self) -> &mut HashMap<AccrGroup, AccrSubmember> {
    &mut self.0 //pointer to Inner value
  }
}
impl Deref for AccrName {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0 //pointer to Inner value
  }
}
impl Deref for AttrName {
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

#[derive(Hash, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Integer(String);

impl From<Integer> for i32 {
  fn from(attr_val: Integer) -> Self {
    match attr_val {
      x => x.0.parse().unwrap(),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttrContent(HashMap<Integer, AttrProperty>);
impl Deref for AttrContent {
  type Target = HashMap<Integer, AttrProperty>;

  fn deref(&self) -> &Self::Target {
    &self.0 //pointer to Inner value
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttrMetadata(HashMap<AttrName, AttrContent>);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttrProperty {
  access: AttrAccess,
  name: String,
  r#enum: Option<AttrEnum>,
  resettable: bool,
  r#type: AttrType,
}

#[derive(Hash, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct AttrAccess(String);
#[derive(Hash, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct AttrResettable(String);
#[derive(Hash, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct AttrType(String);
#[derive(Hash, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct AttrEnum(String);

impl From<AttrAccessEnum> for String {
  fn from(val: AttrAccessEnum) -> String {
    match val {
      AttrAccessEnum::Read => "AttrAccess::Read",
      AttrAccessEnum::Write => "AttrAccess::Write",
      AttrAccessEnum::ReadWrite => "AttrAccess::ReadWrite",
    }
    .to_string()
  }
}

enum AttrAccessEnum {
  Read,
  Write,
  ReadWrite,
}
impl From<AttrType> for (&str, &str) {
  fn from(val: AttrType) -> Self {
    match val.0.as_str() {
      "int8" => ("i8", "0"),
      "uInt8" => ("u8", "0"),
      "int16" => ("i16", "0"),
      "uInt16" => ("u16", "0"),
      "int32" => ("i32", "0"),
      "uInt32" => ("u32", "0"),
      "float32" => ("f32", "0"),
      "float64" => ("f64", "0"),
      "int64" => ("i64", "0"),
      "uInt64" => ("u64", "0"),
      "bool32" => ("bool", "0"),
      "CVIAbsoluteTime" => ("CVIAbsoluteTime", "0"),
      "int8[]" => ("i8", "N"),
      "uInt8[]" => ("u8", "N"),
      "int16[]" => ("i16", "N"),
      "uInt16[]" => ("u16", "N"),
      "int32[]" => ("i32", "N"),
      "uInt32[]" => ("u32", "N"),
      "float32[]" => ("f32", "N"),
      "float64[]" => ("f64", "N"),
      "int64[]" => ("i64", "N"),
      "uInt64[]" => ("u64", "N"),
      "char[]" => ("i8", "N"),
      _ => panic!("Missing match arm detected!"),
    }
  }
}

impl From<String> for AttrAccessEnum {
  fn from(val: String) -> AttrAccessEnum {
    match val.as_str() {
      "read" => AttrAccessEnum::Read,
      "write" => AttrAccessEnum::Write,
      "read-write" => AttrAccessEnum::ReadWrite,
      _ => panic!("Invalid AttrAccess variant"),
    }
  }
}

impl Deref for AttrAccess {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0 //pointer to Inner value
  }
}
impl Deref for AttrResettable {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0 //pointer to Inner value
  }
}
impl Deref for AttrType {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0 //pointer to Inner value
  }
}
impl Default for NidaqmxMetadata {
  fn default() -> NidaqmxMetadata {
    let metadata_dir = PathBuf::from("metadata/nidaqmx/");
    let metadata = [
      "functions.json",
      "attributes.json",
      "enums.json",
      "accessors.json",
    ];
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
    let accr: AccrMetadata =
      serde_json::from_reader(map.get_mut("accessors").unwrap()).unwrap();

    NidaqmxMetadata {
      enums,
      func,
      attr,
      accr,
    }
  }
}

impl ItemProcessor for AttrProperty {
  fn prefix_letter(mut self, letter: &str) -> Self {
    let foo = match self.name.as_bytes()[0].is_ascii_digit() {
      true => letter.to_owned() + &self.name,
      false => self.name,
    };
    self.name = foo;
    self
  }
}

impl ItemProcessor for EnumVariant {
  fn prefix_letter(mut self, letter: &str) -> Self {
    let foo = match self.name.as_bytes()[0].is_ascii_digit() {
      true => letter.to_owned() + &self.name,
      false => self.name,
    };
    self.name = foo;
    self
  }
}

pub trait ItemProcessor {
  fn prefix_letter(self, letter: &str) -> Self;
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

  fn serde_accr() -> Result<AccrMetadata> {
    let file = File::open("metadata/nidaqmx/accessors.json").unwrap();
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
  fn test_accr() {
    let u = serde_accr().unwrap();
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
    println!("{:#?}", variant.prefix_letter("d"));
  }
  #[test]
  fn test_gen_enums() {
    let gen = NidaqmxGen::new();
    let rendered = gen.generate_enums().generate();
    println!("{:#?}", rendered);
  }
}
