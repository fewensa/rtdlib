
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about the current temporary password.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTemporaryPasswordState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getTemporaryPasswordState
  
}



impl Object for GetTemporaryPasswordState {}
impl RObject for GetTemporaryPasswordState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getTemporaryPasswordState" }
  fn td_type(&self) -> RTDType { RTDType::GetTemporaryPasswordState }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetTemporaryPasswordState {}


impl GetTemporaryPasswordState {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getTemporaryPasswordState".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



