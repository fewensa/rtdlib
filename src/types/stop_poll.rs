
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Stops a poll. A poll in a message can be stopped when the message has can_be_edited flag set.
#[derive(Debug, Serialize, Deserialize)]
pub struct StopPoll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // stopPoll
  /// Identifier of the chat to which the poll belongs.
  chat_id: Option<i64>,
  /// Identifier of the message containing the poll.
  message_id: Option<i64>,
  /// The new message reply markup; for bots only.
  reply_markup: Option<Box<ReplyMarkup>>,
  
}


impl Clone for StopPoll {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for StopPoll {}
impl RObject for StopPoll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "stopPoll" }
  fn td_type(&self) -> RTDType { RTDType::StopPoll }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for StopPoll {}


impl StopPoll {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "stopPoll".to_string(),
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



