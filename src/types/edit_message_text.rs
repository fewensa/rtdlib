
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Edits the text of a message (or a text of a game message). Returns the edited message after the edit is completed on the server side.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditMessageText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // editMessageText
  /// The chat the message belongs to.
  chat_id: Option<i64>,
  /// Identifier of the message.
  message_id: Option<i64>,
  /// The new message reply markup; for bots only.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// New text content of the message. Should be of type InputMessageText.
  input_message_content: Option<Box<InputMessageContent>>,
  
}


impl Clone for EditMessageText {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for EditMessageText {}
impl RObject for EditMessageText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editMessageText" }
  fn td_type(&self) -> RTDType { RTDType::EditMessageText }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for EditMessageText {}


impl EditMessageText {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "editMessageText".to_string(),
      chat_id: None,
      message_id: None,
      reply_markup: None,
      input_message_content: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn input_message_content(&self) -> Option<Box<InputMessageContent>> { self.input_message_content.clone() }
  #[doc(hidden)] pub fn _set_input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



