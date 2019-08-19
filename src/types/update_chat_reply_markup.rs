
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The default chat reply markup was changed. Can occur because new messages with reply markup were received or because an old reply markup was hidden by the user. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatReplyMarkup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateChatReplyMarkup
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Identifier of the message from which reply markup needs to be used; 0 if there is no default custom reply markup in the chat.
  reply_markup_message_id: Option<i64>,
  
}



impl Object for UpdateChatReplyMarkup {}
impl RObject for UpdateChatReplyMarkup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatReplyMarkup" }
  fn td_type(&self) -> RTDType { RTDType::UpdateChatReplyMarkup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateChatReplyMarkup {}


impl UpdateChatReplyMarkup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateChatReplyMarkup".to_string(),
      chat_id: None,
      reply_markup_message_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn reply_markup_message_id(&self) -> Option<i64> { self.reply_markup_message_id.clone() }
  #[doc(hidden)] pub fn _set_reply_markup_message_id(&mut self, reply_markup_message_id: i64) -> &mut Self { self.reply_markup_message_id = Some(reply_markup_message_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



