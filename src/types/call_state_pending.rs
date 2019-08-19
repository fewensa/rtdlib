
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The call is pending, waiting to be accepted by a user. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStatePending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callStatePending
  /// True, if the call has already been created by the server.
  is_created: Option<bool>,
  /// True, if the call has already been received by the other party.
  is_received: Option<bool>,
  
}



impl Object for CallStatePending {}
impl RObject for CallStatePending {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callStatePending" }
  fn td_type(&self) -> RTDType { RTDType::CallStatePending }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CallState for CallStatePending {}


impl CallStatePending {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callStatePending".to_string(),
      is_created: None,
      is_received: None,
      
    }
  }
  
  pub fn is_created(&self) -> Option<bool> { self.is_created.clone() }
  #[doc(hidden)] pub fn _set_is_created(&mut self, is_created: bool) -> &mut Self { self.is_created = Some(is_created); self }
  
  pub fn is_received(&self) -> Option<bool> { self.is_received.clone() }
  #[doc(hidden)] pub fn _set_is_received(&mut self, is_received: bool) -> &mut Self { self.is_received = Some(is_received); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



