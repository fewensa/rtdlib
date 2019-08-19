
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Pins a message in a chat; requires appropriate administrator rights in the group or channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinChatMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pinChatMessage
  /// Identifier of the chat.
  chat_id: Option<i64>,
  /// Identifier of the new pinned message.
  message_id: Option<i64>,
  /// True, if there should be no notification about the pinned message.
  disable_notification: Option<bool>,
  
}



impl Object for PinChatMessage {}
impl RObject for PinChatMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pinChatMessage" }
  fn td_type(&self) -> RTDType { RTDType::PinChatMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for PinChatMessage {}


impl PinChatMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pinChatMessage".to_string(),
      chat_id: None,
      message_id: None,
      disable_notification: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn disable_notification(&self) -> Option<bool> { self.disable_notification.clone() }
  #[doc(hidden)] pub fn _set_disable_notification(&mut self, disable_notification: bool) -> &mut Self { self.disable_notification = Some(disable_notification); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



