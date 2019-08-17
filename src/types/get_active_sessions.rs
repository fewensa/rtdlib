
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns all active sessions of the current user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActiveSessions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getActiveSessions
  
}



impl Object for GetActiveSessions {}
impl RObject for GetActiveSessions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getActiveSessions" }
  fn td_type(&self) -> RTDType { RTDType::GetActiveSessions }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetActiveSessions {}


impl GetActiveSessions {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getActiveSessions".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



