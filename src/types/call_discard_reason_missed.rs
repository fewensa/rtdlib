
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The call was ended before the conversation started. It was cancelled by the caller or missed by the other party. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDiscardReasonMissed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callDiscardReasonMissed
  
}



impl Object for CallDiscardReasonMissed {}
impl RObject for CallDiscardReasonMissed {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callDiscardReasonMissed" }
  fn td_type(&self) -> RTDType { RTDType::CallDiscardReasonMissed }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CallDiscardReason for CallDiscardReasonMissed {}


impl CallDiscardReasonMissed {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callDiscardReasonMissed".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



