
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message was edited. Changes in the message content will come in a separate 
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMessageEdited {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateMessageEdited
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Message identifier.
  message_id: Option<i64>,
  /// Point in time (Unix timestamp) when the message was edited.
  edit_date: Option<i32>,
  /// New message reply markup; may be null.
  reply_markup: Option<Box<ReplyMarkup>>,
  
}


impl Clone for UpdateMessageEdited {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UpdateMessageEdited {}
impl RObject for UpdateMessageEdited {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageEdited" }
  fn td_type(&self) -> RTDType { RTDType::UpdateMessageEdited }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateMessageEdited {}


impl UpdateMessageEdited {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateMessageEdited".to_string(),
      chat_id: None,
      message_id: None,
      edit_date: None,
      reply_markup: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn edit_date(&self) -> Option<i32> { self.edit_date.clone() }
  #[doc(hidden)] pub fn _set_edit_date(&mut self, edit_date: i32) -> &mut Self { self.edit_date = Some(edit_date); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



