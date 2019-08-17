
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The chat pinned message was changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatPinnedMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateChatPinnedMessage
  /// Chat identifier.
  chat_id: Option<i64>,
  /// The new identifier of the pinned message; 0 if there is no pinned message in the chat.
  pinned_message_id: Option<i64>,
  
}



impl Object for UpdateChatPinnedMessage {}
impl RObject for UpdateChatPinnedMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatPinnedMessage" }
  fn td_type(&self) -> RTDType { RTDType::UpdateChatPinnedMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateChatPinnedMessage {}


impl UpdateChatPinnedMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateChatPinnedMessage".to_string(),
      chat_id: None,
      pinned_message_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn pinned_message_id(&self) -> Option<i64> { self.pinned_message_id.clone() }
  #[doc(hidden)] pub fn _set_pinned_message_id(&mut self, pinned_message_id: i64) -> &mut Self { self.pinned_message_id = Some(pinned_message_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



