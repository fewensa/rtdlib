
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sets the verbosity level for a specified TDLib internal log tag. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLogTagVerbosityLevel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setLogTagVerbosityLevel
  /// Logging tag to change verbosity level.
  tag: Option<String>,
  /// New verbosity level; 1-1024.
  new_verbosity_level: Option<i32>,
  
}



impl Object for SetLogTagVerbosityLevel {}
impl RObject for SetLogTagVerbosityLevel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setLogTagVerbosityLevel" }
  fn td_type(&self) -> RTDType { RTDType::SetLogTagVerbosityLevel }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetLogTagVerbosityLevel {}


impl SetLogTagVerbosityLevel {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setLogTagVerbosityLevel".to_string(),
      tag: None,
      new_verbosity_level: None,
      
    }
  }
  
  pub fn tag(&self) -> Option<String> { self.tag.clone() }
  #[doc(hidden)] pub fn _set_tag(&mut self, tag: String) -> &mut Self { self.tag = Some(tag); self }
  
  pub fn new_verbosity_level(&self) -> Option<i32> { self.new_verbosity_level.clone() }
  #[doc(hidden)] pub fn _set_new_verbosity_level(&mut self, new_verbosity_level: i32) -> &mut Self { self.new_verbosity_level = Some(new_verbosity_level); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



