
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The sign_messages setting of a channel was toggled. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventSignMessagesToggled {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventSignMessagesToggled
  /// New value of sign_messages.
  sign_messages: Option<bool>,
  
}



impl Object for ChatEventSignMessagesToggled {}
impl RObject for ChatEventSignMessagesToggled {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventSignMessagesToggled" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventSignMessagesToggled }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventSignMessagesToggled {}


impl ChatEventSignMessagesToggled {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventSignMessagesToggled".to_string(),
      sign_messages: None,
      
    }
  }
  
  pub fn sign_messages(&self) -> Option<bool> { self.sign_messages.clone() }
  #[doc(hidden)] pub fn _set_sign_messages(&mut self, sign_messages: bool) -> &mut Self { self.sign_messages = Some(sign_messages); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



