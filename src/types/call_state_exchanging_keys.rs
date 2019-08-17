
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The call has been answered and encryption keys are being exchanged. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStateExchangingKeys {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callStateExchangingKeys
  
}



impl Object for CallStateExchangingKeys {}
impl RObject for CallStateExchangingKeys {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callStateExchangingKeys" }
  fn td_type(&self) -> RTDType { RTDType::CallStateExchangingKeys }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CallState for CallStateExchangingKeys {}


impl CallStateExchangingKeys {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callStateExchangingKeys".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



