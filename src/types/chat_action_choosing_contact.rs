
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is picking a contact to send. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatActionChoosingContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatActionChoosingContact
  
}



impl Object for ChatActionChoosingContact {}
impl RObject for ChatActionChoosingContact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionChoosingContact" }
  fn td_type(&self) -> RTDType { RTDType::ChatActionChoosingContact }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatAction for ChatActionChoosingContact {}


impl ChatActionChoosingContact {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatActionChoosingContact".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



