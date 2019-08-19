
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Incoming messages were read or number of unread messages has been changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatReadInbox {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateChatReadInbox
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Identifier of the last read incoming message.
  last_read_inbox_message_id: Option<i64>,
  /// The number of unread messages left in the chat.
  unread_count: Option<i32>,
  
}



impl Object for UpdateChatReadInbox {}
impl RObject for UpdateChatReadInbox {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatReadInbox" }
  fn td_type(&self) -> RTDType { RTDType::UpdateChatReadInbox }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateChatReadInbox {}


impl UpdateChatReadInbox {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateChatReadInbox".to_string(),
      chat_id: None,
      last_read_inbox_message_id: None,
      unread_count: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn last_read_inbox_message_id(&self) -> Option<i64> { self.last_read_inbox_message_id.clone() }
  #[doc(hidden)] pub fn _set_last_read_inbox_message_id(&mut self, last_read_inbox_message_id: i64) -> &mut Self { self.last_read_inbox_message_id = Some(last_read_inbox_message_id); self }
  
  pub fn unread_count(&self) -> Option<i32> { self.unread_count.clone() }
  #[doc(hidden)] pub fn _set_unread_count(&mut self, unread_count: i32) -> &mut Self { self.unread_count = Some(unread_count); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



