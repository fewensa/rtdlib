
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message has been successfully sent. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessageSendSucceeded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateMessageSendSucceeded
  /// Information about the sent message. Usually only the message identifier, date, and content are changed, but almost all other fields can also change.
  message: Option<Message>,
  /// The previous temporary message identifier.
  old_message_id: Option<i64>,
  
}



impl Object for UpdateMessageSendSucceeded {}
impl RObject for UpdateMessageSendSucceeded {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageSendSucceeded" }
  fn td_type(&self) -> RTDType { RTDType::UpdateMessageSendSucceeded }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateMessageSendSucceeded {}


impl UpdateMessageSendSucceeded {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateMessageSendSucceeded".to_string(),
      message: None,
      old_message_id: None,
      
    }
  }
  
  pub fn message(&self) -> Option<Message> { self.message.clone() }
  #[doc(hidden)] pub fn _set_message(&mut self, message: Message) -> &mut Self { self.message = Some(message); self }
  
  pub fn old_message_id(&self) -> Option<i64> { self.old_message_id.clone() }
  #[doc(hidden)] pub fn _set_old_message_id(&mut self, old_message_id: i64) -> &mut Self { self.old_message_id = Some(old_message_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



