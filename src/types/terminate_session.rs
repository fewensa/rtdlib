
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Terminates a session of the current user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateSession {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // terminateSession
  /// Session identifier.
  session_id: Option<i64>,
  
}



impl Object for TerminateSession {}
impl RObject for TerminateSession {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "terminateSession" }
  fn td_type(&self) -> RTDType { RTDType::TerminateSession }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for TerminateSession {}


impl TerminateSession {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "terminateSession".to_string(),
      session_id: None,
      
    }
  }
  
  pub fn session_id(&self) -> Option<i64> { self.session_id.clone() }
  #[doc(hidden)] pub fn _set_session_id(&mut self, session_id: i64) -> &mut Self { self.session_id = Some(session_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



