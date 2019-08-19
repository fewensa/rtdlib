
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The call wasn't discarded, or the reason is unknown. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDiscardReasonEmpty {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callDiscardReasonEmpty
  
}



impl Object for CallDiscardReasonEmpty {}
impl RObject for CallDiscardReasonEmpty {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callDiscardReasonEmpty" }
  fn td_type(&self) -> RTDType { RTDType::CallDiscardReasonEmpty }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CallDiscardReason for CallDiscardReasonEmpty {}


impl CallDiscardReasonEmpty {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callDiscardReasonEmpty".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



