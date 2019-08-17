
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Removes the pinned message from a chat; requires appropriate administrator rights in the group or channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnpinChatMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // unpinChatMessage
  /// Identifier of the chat.
  chat_id: Option<i64>,
  
}



impl Object for UnpinChatMessage {}
impl RObject for UnpinChatMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "unpinChatMessage" }
  fn td_type(&self) -> RTDType { RTDType::UnpinChatMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for UnpinChatMessage {}


impl UnpinChatMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "unpinChatMessage".to_string(),
      chat_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



