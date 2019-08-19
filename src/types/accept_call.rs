
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Accepts an incoming call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // acceptCall
  /// Call identifier.
  call_id: Option<i32>,
  /// Description of the call protocols supported by the client.
  protocol: Option<CallProtocol>,
  
}



impl Object for AcceptCall {}
impl RObject for AcceptCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "acceptCall" }
  fn td_type(&self) -> RTDType { RTDType::AcceptCall }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AcceptCall {}


impl AcceptCall {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "acceptCall".to_string(),
      call_id: None,
      protocol: None,
      
    }
  }
  
  pub fn call_id(&self) -> Option<i32> { self.call_id.clone() }
  #[doc(hidden)] pub fn _set_call_id(&mut self, call_id: i32) -> &mut Self { self.call_id = Some(call_id); self }
  
  pub fn protocol(&self) -> Option<CallProtocol> { self.protocol.clone() }
  #[doc(hidden)] pub fn _set_protocol(&mut self, protocol: CallProtocol) -> &mut Self { self.protocol = Some(protocol); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



