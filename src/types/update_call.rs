
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// New call was created or information about a call was updated. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateCall
  /// New data about a call.
  call: Option<Call>,
  
}



impl Object for UpdateCall {}
impl RObject for UpdateCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateCall" }
  fn td_type(&self) -> RTDType { RTDType::UpdateCall }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateCall {}


impl UpdateCall {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateCall".to_string(),
      call: None,
      
    }
  }
  
  pub fn call(&self) -> Option<Call> { self.call.clone() }
  #[doc(hidden)] pub fn _set_call(&mut self, call: Call) -> &mut Self { self.call = Some(call); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



