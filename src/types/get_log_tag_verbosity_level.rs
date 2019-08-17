
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns current verbosity level for a specified TDLib internal log tag. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLogTagVerbosityLevel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getLogTagVerbosityLevel
  /// Logging tag to change verbosity level.
  tag: Option<String>,
  
}



impl Object for GetLogTagVerbosityLevel {}
impl RObject for GetLogTagVerbosityLevel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLogTagVerbosityLevel" }
  fn td_type(&self) -> RTDType { RTDType::GetLogTagVerbosityLevel }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetLogTagVerbosityLevel {}


impl GetLogTagVerbosityLevel {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getLogTagVerbosityLevel".to_string(),
      tag: None,
      
    }
  }
  
  pub fn tag(&self) -> Option<String> { self.tag.clone() }
  #[doc(hidden)] pub fn _set_tag(&mut self, tag: String) -> &mut Self { self.tag = Some(tag); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



