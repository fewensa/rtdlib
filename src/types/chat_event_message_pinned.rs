
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message was pinned. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventMessagePinned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventMessagePinned
  /// Pinned message.
  message: Option<Message>,
  
}



impl Object for ChatEventMessagePinned {}
impl RObject for ChatEventMessagePinned {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMessagePinned" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventMessagePinned }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventMessagePinned {}


impl ChatEventMessagePinned {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventMessagePinned".to_string(),
      message: None,
      
    }
  }
  
  pub fn message(&self) -> Option<Message> { self.message.clone() }
  #[doc(hidden)] pub fn _set_message(&mut self, message: Message) -> &mut Self { self.message = Some(message); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



