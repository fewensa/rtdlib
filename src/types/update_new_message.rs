
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new message was received; can also be an outgoing message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNewMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateNewMessage
  /// The new message.
  message: Option<Message>,
  
}



impl Object for UpdateNewMessage {}
impl RObject for UpdateNewMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewMessage" }
  fn td_type(&self) -> RTDType { RTDType::UpdateNewMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateNewMessage {}


impl UpdateNewMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateNewMessage".to_string(),
      message: None,
      
    }
  }
  
  pub fn message(&self) -> Option<Message> { self.message.clone() }
  #[doc(hidden)] pub fn _set_message(&mut self, message: Message) -> &mut Self { self.message = Some(message); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



