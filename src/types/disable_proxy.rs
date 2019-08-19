
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Disables the currently enabled proxy. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisableProxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // disableProxy
  
}



impl Object for DisableProxy {}
impl RObject for DisableProxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "disableProxy" }
  fn td_type(&self) -> RTDType { RTDType::DisableProxy }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DisableProxy {}


impl DisableProxy {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "disableProxy".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



