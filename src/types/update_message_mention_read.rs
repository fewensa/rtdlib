
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with an unread mention was read. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessageMentionRead {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateMessageMentionRead
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Message identifier.
  message_id: Option<i64>,
  /// The new number of unread mention messages left in the chat.
  unread_mention_count: Option<i32>,
  
}



impl Object for UpdateMessageMentionRead {}
impl RObject for UpdateMessageMentionRead {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageMentionRead" }
  fn td_type(&self) -> RTDType { RTDType::UpdateMessageMentionRead }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateMessageMentionRead {}


impl UpdateMessageMentionRead {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateMessageMentionRead".to_string(),
      chat_id: None,
      message_id: None,
      unread_mention_count: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn unread_mention_count(&self) -> Option<i32> { self.unread_mention_count.clone() }
  #[doc(hidden)] pub fn _set_unread_mention_count(&mut self, unread_mention_count: i32) -> &mut Self { self.unread_mention_count = Some(unread_mention_count); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



