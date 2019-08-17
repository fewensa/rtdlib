
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a call. 
#[derive(Debug, Serialize, Deserialize)]
pub struct Call {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // call
  /// Call identifier, not persistent.
  id: Option<i32>,
  /// Peer user identifier.
  user_id: Option<i32>,
  /// True, if the call is outgoing.
  is_outgoing: Option<bool>,
  /// Call state.
  state: Option<Box<CallState>>,
  
}


impl Clone for Call {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for Call {}
impl RObject for Call {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "call" }
  fn td_type(&self) -> RTDType { RTDType::Call }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Call {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "call".to_string(),
      id: None,
      user_id: None,
      is_outgoing: None,
      state: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i32> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i32) -> &mut Self { self.id = Some(id); self }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn is_outgoing(&self) -> Option<bool> { self.is_outgoing.clone() }
  #[doc(hidden)] pub fn _set_is_outgoing(&mut self, is_outgoing: bool) -> &mut Self { self.is_outgoing = Some(is_outgoing); self }
  
  pub fn state(&self) -> Option<Box<CallState>> { self.state.clone() }
  #[doc(hidden)] pub fn _set_state(&mut self, state: Box<CallState>) -> &mut Self { self.state = Some(state); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



