
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessageSendFailed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateMessageSendFailed
  /// Contains information about the message that failed to send.
  message: Option<Message>,
  /// The previous temporary message identifier.
  old_message_id: Option<i64>,
  /// An error code.
  error_code: Option<i32>,
  /// Error message.
  error_message: Option<String>,
  
}



impl Object for UpdateMessageSendFailed {}
impl RObject for UpdateMessageSendFailed {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageSendFailed" }
  fn td_type(&self) -> RTDType { RTDType::UpdateMessageSendFailed }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateMessageSendFailed {}


impl UpdateMessageSendFailed {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateMessageSendFailed".to_string(),
      message: None,
      old_message_id: None,
      error_code: None,
      error_message: None,
      
    }
  }
  
  pub fn message(&self) -> Option<Message> { self.message.clone() }
  #[doc(hidden)] pub fn _set_message(&mut self, message: Message) -> &mut Self { self.message = Some(message); self }
  
  pub fn old_message_id(&self) -> Option<i64> { self.old_message_id.clone() }
  #[doc(hidden)] pub fn _set_old_message_id(&mut self, old_message_id: i64) -> &mut Self { self.old_message_id = Some(old_message_id); self }
  
  pub fn error_code(&self) -> Option<i32> { self.error_code.clone() }
  #[doc(hidden)] pub fn _set_error_code(&mut self, error_code: i32) -> &mut Self { self.error_code = Some(error_code); self }
  
  pub fn error_message(&self) -> Option<String> { self.error_message.clone() }
  #[doc(hidden)] pub fn _set_error_message(&mut self, error_message: String) -> &mut Self { self.error_message = Some(error_message); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



