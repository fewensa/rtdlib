
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a list of sessions. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sessions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sessions
  /// List of sessions.
  sessions: Option<Vec<Session>>,
  
}



impl Object for Sessions {}
impl RObject for Sessions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sessions" }
  fn td_type(&self) -> RTDType { RTDType::Sessions }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Sessions {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sessions".to_string(),
      sessions: None,
      
    }
  }
  
  pub fn sessions(&self) -> Option<Vec<Session>> { self.sessions.clone() }
  #[doc(hidden)] pub fn _set_sessions(&mut self, sessions: Vec<Session>) -> &mut Self { self.sessions = Some(sessions); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



