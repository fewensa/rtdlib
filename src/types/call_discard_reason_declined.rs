
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The call was ended before the conversation started. It was declined by the other party. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDiscardReasonDeclined {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callDiscardReasonDeclined
  
}



impl Object for CallDiscardReasonDeclined {}
impl RObject for CallDiscardReasonDeclined {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callDiscardReasonDeclined" }
  fn td_type(&self) -> RTDType { RTDType::CallDiscardReasonDeclined }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CallDiscardReason for CallDiscardReasonDeclined {}


impl CallDiscardReasonDeclined {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callDiscardReasonDeclined".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



