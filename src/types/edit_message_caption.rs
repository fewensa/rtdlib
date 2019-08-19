
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Edits the message content caption. Returns the edited message after the edit is completed on the server side.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditMessageCaption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // editMessageCaption
  /// The chat the message belongs to.
  chat_id: Option<i64>,
  /// Identifier of the message.
  message_id: Option<i64>,
  /// The new message reply markup; for bots only.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// New message content caption; 0-GetOption("message_caption_length_max") characters.
  caption: Option<FormattedText>,
  
}


impl Clone for EditMessageCaption {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for EditMessageCaption {}
impl RObject for EditMessageCaption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editMessageCaption" }
  fn td_type(&self) -> RTDType { RTDType::EditMessageCaption }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for EditMessageCaption {}


impl EditMessageCaption {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "editMessageCaption".to_string(),
      chat_id: None,
      message_id: None,
      reply_markup: None,
      caption: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn caption(&self) -> Option<FormattedText> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



