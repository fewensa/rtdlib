
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The call is hanging up after 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStateHangingUp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callStateHangingUp
  
}



impl Object for CallStateHangingUp {}
impl RObject for CallStateHangingUp {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callStateHangingUp" }
  fn td_type(&self) -> RTDType { RTDType::CallStateHangingUp }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CallState for CallStateHangingUp {}


impl CallStateHangingUp {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callStateHangingUp".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



