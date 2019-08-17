
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user has cancelled the previous action. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatActionCancel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatActionCancel
  
}



impl Object for ChatActionCancel {}
impl RObject for ChatActionCancel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionCancel" }
  fn td_type(&self) -> RTDType { RTDType::ChatActionCancel }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatAction for ChatActionCancel {}


impl ChatActionCancel {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatActionCancel".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



