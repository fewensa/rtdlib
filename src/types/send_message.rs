
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends a message. Returns the sent message.
#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sendMessage
  /// Target chat.
  chat_id: Option<i64>,
  /// Identifier of the message to reply to or 0.
  reply_to_message_id: Option<i64>,
  /// Pass true to disable notification for the message. Not supported in secret chats.
  disable_notification: Option<bool>,
  /// Pass true if the message is sent from the background.
  from_background: Option<bool>,
  /// Markup for replying to the message; for bots only.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// The content of the message to be sent.
  input_message_content: Option<Box<InputMessageContent>>,
  
}


impl Clone for SendMessage {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SendMessage {}
impl RObject for SendMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendMessage" }
  fn td_type(&self) -> RTDType { RTDType::SendMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SendMessage {}


impl SendMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sendMessage".to_string(),
      chat_id: None,
      reply_to_message_id: None,
      disable_notification: None,
      from_background: None,
      reply_markup: None,
      input_message_content: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn reply_to_message_id(&self) -> Option<i64> { self.reply_to_message_id.clone() }
  #[doc(hidden)] pub fn _set_reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self { self.reply_to_message_id = Some(reply_to_message_id); self }
  
  pub fn disable_notification(&self) -> Option<bool> { self.disable_notification.clone() }
  #[doc(hidden)] pub fn _set_disable_notification(&mut self, disable_notification: bool) -> &mut Self { self.disable_notification = Some(disable_notification); self }
  
  pub fn from_background(&self) -> Option<bool> { self.from_background.clone() }
  #[doc(hidden)] pub fn _set_from_background(&mut self, from_background: bool) -> &mut Self { self.from_background = Some(from_background); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn input_message_content(&self) -> Option<Box<InputMessageContent>> { self.input_message_content.clone() }
  #[doc(hidden)] pub fn _set_input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



