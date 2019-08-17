
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the current state of 2-step verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPasswordState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getPasswordState
  
}



impl Object for GetPasswordState {}
impl RObject for GetPasswordState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPasswordState" }
  fn td_type(&self) -> RTDType { RTDType::GetPasswordState }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetPasswordState {}


impl GetPasswordState {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getPasswordState".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



