use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// td type fill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TDTypeFill {
  /// origin type mapper to rust
  mapper: HashMap<String, String>,
  /// type filter
  filter: HashMap<String, HashMap<String, TDTypeFilter>>,
  /// addition listener
  listener: HashMap<String, String>,
}

impl TDTypeFill {
  pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, failure::Error> {
    let path = path.as_ref();
    if !path.exists() { return bail!("type fill file not found"); }
    if !path.is_file() { return bail!("type fill path is not a file"); }
    let toml_text = std::fs::read_to_string(path)?;
    let tdf: toml::Value = toml::from_str(&toml_text[..]).unwrap();
    let tdtypefill = match toml::from_str(&toml_text[..]) {
      Ok(t) => t,
      Err(e) => return bail!("Can not convert type fill file -> {:?}", e)
    };
//    println!("{:#?}", tdtypefill);
    Ok(tdtypefill)
  }

  pub fn mapper<S: AsRef<str>>(&self, origin: S) -> Option<String> {
    self.mapper.get(origin.as_ref()).map(|v| v.to_string())
  }

  pub fn td_filter<S0: AsRef<str>, S1: AsRef<str>>(
    &self, type_name: S0, field_name: S1,
  ) -> Option<TDTypeFilter> {
    self.filter.keys()
      .find(|&key| key.to_lowercase() == type_name.as_ref().to_lowercase())
      .map(|key| self.filter.get(key).map(|v| {
        v.keys()
          .find(|&fkey| fkey.to_lowercase() == field_name.as_ref().to_lowercase())
          .map(|fkey| v.get(fkey).map(|v| v.clone()))
      }))
      .map_or(None, |v| v)
      .map_or(None, |v| v)
      .map_or(None, |v| v)
  }

  pub fn td_filter_type<S0: AsRef<str>, S1: AsRef<str>, S2: AsRef<str>>
  (
    &self,
    type_name: S0,
    field_name: S1,
    origin_field_type: S2,
  ) -> String {
    let origin_field_type = origin_field_type.as_ref();
    self.td_filter(type_name, field_name)
      .map_or(
        origin_field_type.to_string(),
        |v| v.sign_type()
          .filter(|v| !v.is_empty())
          .map_or(origin_field_type.to_string(), |v| v),
      )
  }

  pub fn td_filter_macros<S0: AsRef<str>, S1: AsRef<str>>(
    &self,
    type_name: S0,
    field_name: S1,
  ) -> Vec<String> {
    self.td_filter(type_name, field_name)
      .map_or(
        vec![],
        |v| v.macros()
          .filter(|macros| !macros.is_empty())
          .map_or(vec![], |macros| macros)
      )
  }

  pub fn listener(&self) -> &HashMap<String, String> {
    &self.listener
  }
}


/// td field type filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TDTypeFilter {
  sign_type: Option<String>,
  macros: Option<Vec<String>>,
  optional: bool,
  reason: Option<String>,
}

impl TDTypeFilter {
  pub fn sign_type(&self) -> Option<String> { self.sign_type.clone() }
  pub fn optional(&self) -> bool { self.optional }
  pub fn reason(&self) -> Option<String> { self.reason.clone() }
  pub fn macros(&self) -> Option<Vec<String>> { self.macros.clone() }
}




