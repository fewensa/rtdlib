
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is picking a location or venue to send. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatActionChoosingLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatActionChoosingLocation
  
}



impl Object for ChatActionChoosingLocation {}
impl RObject for ChatActionChoosingLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionChoosingLocation" }
  fn td_type(&self) -> RTDType { RTDType::ChatActionChoosingLocation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatAction for ChatActionChoosingLocation {}


impl ChatActionChoosingLocation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatActionChoosingLocation".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



