
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about a pinned chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatPinnedMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getChatPinnedMessage
  /// Identifier of the chat the message belongs to.
  chat_id: Option<i64>,
  
}



impl Object for GetChatPinnedMessage {}
impl RObject for GetChatPinnedMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatPinnedMessage" }
  fn td_type(&self) -> RTDType { RTDType::GetChatPinnedMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetChatPinnedMessage {}


impl GetChatPinnedMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getChatPinnedMessage".to_string(),
      chat_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



