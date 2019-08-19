
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The call was ended because one of the parties hung up. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDiscardReasonHungUp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callDiscardReasonHungUp
  
}



impl Object for CallDiscardReasonHungUp {}
impl RObject for CallDiscardReasonHungUp {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callDiscardReasonHungUp" }
  fn td_type(&self) -> RTDType { RTDType::CallDiscardReasonHungUp }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CallDiscardReason for CallDiscardReasonHungUp {}


impl CallDiscardReasonHungUp {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callDiscardReasonHungUp".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



