
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The call has ended successfully. 
#[derive(Debug, Serialize, Deserialize)]
pub struct CallStateDiscarded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callStateDiscarded
  /// The reason, why the call has ended.
  reason: Option<Box<CallDiscardReason>>,
  /// True, if the call rating should be sent to the server.
  need_rating: Option<bool>,
  /// True, if the call debug information should be sent to the server.
  need_debug_information: Option<bool>,
  
}


impl Clone for CallStateDiscarded {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for CallStateDiscarded {}
impl RObject for CallStateDiscarded {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callStateDiscarded" }
  fn td_type(&self) -> RTDType { RTDType::CallStateDiscarded }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CallState for CallStateDiscarded {}


impl CallStateDiscarded {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callStateDiscarded".to_string(),
      reason: None,
      need_rating: None,
      need_debug_information: None,
      
    }
  }
  
  pub fn reason(&self) -> Option<Box<CallDiscardReason>> { self.reason.clone() }
  #[doc(hidden)] pub fn _set_reason(&mut self, reason: Box<CallDiscardReason>) -> &mut Self { self.reason = Some(reason); self }
  
  pub fn need_rating(&self) -> Option<bool> { self.need_rating.clone() }
  #[doc(hidden)] pub fn _set_need_rating(&mut self, need_rating: bool) -> &mut Self { self.need_rating = Some(need_rating); self }
  
  pub fn need_debug_information(&self) -> Option<bool> { self.need_debug_information.clone() }
  #[doc(hidden)] pub fn _set_need_debug_information(&mut self, need_debug_information: bool) -> &mut Self { self.need_debug_information = Some(need_debug_information); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



