
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends debug information for a call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendCallDebugInformation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sendCallDebugInformation
  /// Call identifier.
  call_id: Option<i32>,
  /// Debug information in application-specific format.
  debug_information: Option<String>,
  
}



impl Object for SendCallDebugInformation {}
impl RObject for SendCallDebugInformation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendCallDebugInformation" }
  fn td_type(&self) -> RTDType { RTDType::SendCallDebugInformation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SendCallDebugInformation {}


impl SendCallDebugInformation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sendCallDebugInformation".to_string(),
      call_id: None,
      debug_information: None,
      
    }
  }
  
  pub fn call_id(&self) -> Option<i32> { self.call_id.clone() }
  #[doc(hidden)] pub fn _set_call_id(&mut self, call_id: i32) -> &mut Self { self.call_id = Some(call_id); self }
  
  pub fn debug_information(&self) -> Option<String> { self.debug_information.clone() }
  #[doc(hidden)] pub fn _set_debug_information(&mut self, debug_information: String) -> &mut Self { self.debug_information = Some(debug_information); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



