
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Deletes the default reply markup from a chat. Must be called after a one-time keyboard or a ForceReply reply markup has been used. UpdateChatReplyMarkup will be sent if the reply markup will be changed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteChatReplyMarkup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deleteChatReplyMarkup
  /// Chat identifier.
  chat_id: Option<i64>,
  /// The message identifier of the used keyboard.
  message_id: Option<i64>,
  
}



impl Object for DeleteChatReplyMarkup {}
impl RObject for DeleteChatReplyMarkup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteChatReplyMarkup" }
  fn td_type(&self) -> RTDType { RTDType::DeleteChatReplyMarkup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DeleteChatReplyMarkup {}


impl DeleteChatReplyMarkup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deleteChatReplyMarkup".to_string(),
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



