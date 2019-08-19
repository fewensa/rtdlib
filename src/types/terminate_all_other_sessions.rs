
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Terminates all other sessions of the current user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateAllOtherSessions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // terminateAllOtherSessions
  
}



impl Object for TerminateAllOtherSessions {}
impl RObject for TerminateAllOtherSessions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "terminateAllOtherSessions" }
  fn td_type(&self) -> RTDType { RTDType::TerminateAllOtherSessions }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for TerminateAllOtherSessions {}


impl TerminateAllOtherSessions {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "terminateAllOtherSessions".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



