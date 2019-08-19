
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Discards a call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscardCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // discardCall
  /// Call identifier.
  call_id: Option<i32>,
  /// True, if the user was disconnected.
  is_disconnected: Option<bool>,
  /// The call duration, in seconds.
  duration: Option<i32>,
  /// Identifier of the connection used during the call.
  connection_id: Option<i64>,
  
}



impl Object for DiscardCall {}
impl RObject for DiscardCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "discardCall" }
  fn td_type(&self) -> RTDType { RTDType::DiscardCall }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DiscardCall {}


impl DiscardCall {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "discardCall".to_string(),
      call_id: None,
      is_disconnected: None,
      duration: None,
      connection_id: None,
      
    }
  }
  
  pub fn call_id(&self) -> Option<i32> { self.call_id.clone() }
  #[doc(hidden)] pub fn _set_call_id(&mut self, call_id: i32) -> &mut Self { self.call_id = Some(call_id); self }
  
  pub fn is_disconnected(&self) -> Option<bool> { self.is_disconnected.clone() }
  #[doc(hidden)] pub fn _set_is_disconnected(&mut self, is_disconnected: bool) -> &mut Self { self.is_disconnected = Some(is_disconnected); self }
  
  pub fn duration(&self) -> Option<i32> { self.duration.clone() }
  #[doc(hidden)] pub fn _set_duration(&mut self, duration: i32) -> &mut Self { self.duration = Some(duration); self }
  
  pub fn connection_id(&self) -> Option<i64> { self.connection_id.clone() }
  #[doc(hidden)] pub fn _set_connection_id(&mut self, connection_id: i64) -> &mut Self { self.connection_id = Some(connection_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



