
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Edits the message reply markup; for bots only. Returns the edited message after the edit is completed on the server side.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditMessageReplyMarkup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // editMessageReplyMarkup
  /// The chat the message belongs to.
  chat_id: Option<i64>,
  /// Identifier of the message.
  message_id: Option<i64>,
  /// The new message reply markup.
  reply_markup: Option<Box<ReplyMarkup>>,
  
}


impl Clone for EditMessageReplyMarkup {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for EditMessageReplyMarkup {}
impl RObject for EditMessageReplyMarkup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editMessageReplyMarkup" }
  fn td_type(&self) -> RTDType { RTDType::EditMessageReplyMarkup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for EditMessageReplyMarkup {}


impl EditMessageReplyMarkup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "editMessageReplyMarkup".to_string(),
      chat_id: None,
      message_id: None,
      reply_markup: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



