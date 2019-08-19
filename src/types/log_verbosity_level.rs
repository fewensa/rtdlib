
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a TDLib internal log verbosity level. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogVerbosityLevel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // logVerbosityLevel
  /// Log verbosity level.
  verbosity_level: Option<i32>,
  
}



impl Object for LogVerbosityLevel {}
impl RObject for LogVerbosityLevel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "logVerbosityLevel" }
  fn td_type(&self) -> RTDType { RTDType::LogVerbosityLevel }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl LogVerbosityLevel {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "logVerbosityLevel".to_string(),
      verbosity_level: None,
      
    }
  }
  
  pub fn verbosity_level(&self) -> Option<i32> { self.verbosity_level.clone() }
  #[doc(hidden)] pub fn _set_verbosity_level(&mut self, verbosity_level: i32) -> &mut Self { self.verbosity_level = Some(verbosity_level); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



