
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns current verbosity level of the internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLogVerbosityLevel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getLogVerbosityLevel
  
}



impl Object for GetLogVerbosityLevel {}
impl RObject for GetLogVerbosityLevel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLogVerbosityLevel" }
  fn td_type(&self) -> RTDType { RTDType::GetLogVerbosityLevel }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetLogVerbosityLevel {}


impl GetLogVerbosityLevel {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getLogVerbosityLevel".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



