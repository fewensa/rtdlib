
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The call was ended during the conversation because the users were disconnected. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDiscardReasonDisconnected {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callDiscardReasonDisconnected
  
}



impl Object for CallDiscardReasonDisconnected {}
impl RObject for CallDiscardReasonDisconnected {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callDiscardReasonDisconnected" }
  fn td_type(&self) -> RTDType { RTDType::CallDiscardReasonDisconnected }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CallDiscardReason for CallDiscardReasonDisconnected {}


impl CallDiscardReasonDisconnected {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callDiscardReasonDisconnected".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



