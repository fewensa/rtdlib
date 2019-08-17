
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with information about an ended call. 
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageCall
  /// Reason why the call was discarded.
  discard_reason: Option<Box<CallDiscardReason>>,
  /// Call duration, in seconds.
  duration: Option<i32>,
  
}


impl Clone for MessageCall {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for MessageCall {}
impl RObject for MessageCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageCall" }
  fn td_type(&self) -> RTDType { RTDType::MessageCall }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageCall {}


impl MessageCall {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageCall".to_string(),
      discard_reason: None,
      duration: None,
      
    }
  }
  
  pub fn discard_reason(&self) -> Option<Box<CallDiscardReason>> { self.discard_reason.clone() }
  #[doc(hidden)] pub fn _set_discard_reason(&mut self, discard_reason: Box<CallDiscardReason>) -> &mut Self { self.discard_reason = Some(discard_reason); self }
  
  pub fn duration(&self) -> Option<i32> { self.duration.clone() }
  #[doc(hidden)] pub fn _set_duration(&mut self, duration: i32) -> &mut Self { self.duration = Some(duration); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



