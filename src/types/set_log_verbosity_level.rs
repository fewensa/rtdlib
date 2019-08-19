
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sets the verbosity level of the internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLogVerbosityLevel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setLogVerbosityLevel
  /// New value of the verbosity level for logging. Value 0 corresponds to fatal errors, value 1 corresponds to errors, value 2 corresponds to warnings and debug warnings, value 3 corresponds to informational, value 4 corresponds to debug, value 5 corresponds to verbose debug, value greater than 5 and up to 1023 can be used to enable even more logging.
  new_verbosity_level: Option<i32>,
  
}



impl Object for SetLogVerbosityLevel {}
impl RObject for SetLogVerbosityLevel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setLogVerbosityLevel" }
  fn td_type(&self) -> RTDType { RTDType::SetLogVerbosityLevel }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetLogVerbosityLevel {}


impl SetLogVerbosityLevel {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setLogVerbosityLevel".to_string(),
      new_verbosity_level: None,
      
    }
  }
  
  pub fn new_verbosity_level(&self) -> Option<i32> { self.new_verbosity_level.clone() }
  #[doc(hidden)] pub fn _set_new_verbosity_level(&mut self, new_verbosity_level: i32) -> &mut Self { self.new_verbosity_level = Some(new_verbosity_level); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



