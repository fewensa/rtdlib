
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the current authorization state; this is an offline request. For informational purposes only. Use 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthorizationState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getAuthorizationState
  
}



impl Object for GetAuthorizationState {}
impl RObject for GetAuthorizationState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getAuthorizationState" }
  fn td_type(&self) -> RTDType { RTDType::GetAuthorizationState }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetAuthorizationState {}


impl GetAuthorizationState {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getAuthorizationState".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



