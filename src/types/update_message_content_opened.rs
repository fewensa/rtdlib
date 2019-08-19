
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the TTL timer for self-destructing messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessageContentOpened {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateMessageContentOpened
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Message identifier.
  message_id: Option<i64>,
  
}



impl Object for UpdateMessageContentOpened {}
impl RObject for UpdateMessageContentOpened {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageContentOpened" }
  fn td_type(&self) -> RTDType { RTDType::UpdateMessageContentOpened }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateMessageContentOpened {}


impl UpdateMessageContentOpened {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateMessageContentOpened".to_string(),
      chat_id: None,
      message_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



