
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds a local message to a chat. The message is persistent across application restarts only if the message database is used. Returns the added message.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddLocalMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // addLocalMessage
  /// Target chat.
  chat_id: Option<i64>,
  /// Identifier of the user who will be shown as the sender of the message; may be 0 for channel posts.
  sender_user_id: Option<i32>,
  /// Identifier of the message to reply to or 0.
  reply_to_message_id: Option<i64>,
  /// Pass true to disable notification for the message.
  disable_notification: Option<bool>,
  /// The content of the message to be added.
  input_message_content: Option<Box<InputMessageContent>>,
  
}


impl Clone for AddLocalMessage {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for AddLocalMessage {}
impl RObject for AddLocalMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addLocalMessage" }
  fn td_type(&self) -> RTDType { RTDType::AddLocalMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AddLocalMessage {}


impl AddLocalMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "addLocalMessage".to_string(),
      chat_id: None,
      sender_user_id: None,
      reply_to_message_id: None,
      disable_notification: None,
      input_message_content: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn sender_user_id(&self) -> Option<i32> { self.sender_user_id.clone() }
  #[doc(hidden)] pub fn _set_sender_user_id(&mut self, sender_user_id: i32) -> &mut Self { self.sender_user_id = Some(sender_user_id); self }
  
  pub fn reply_to_message_id(&self) -> Option<i64> { self.reply_to_message_id.clone() }
  #[doc(hidden)] pub fn _set_reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self { self.reply_to_message_id = Some(reply_to_message_id); self }
  
  pub fn disable_notification(&self) -> Option<bool> { self.disable_notification.clone() }
  #[doc(hidden)] pub fn _set_disable_notification(&mut self, disable_notification: bool) -> &mut Self { self.disable_notification = Some(disable_notification); self }
  
  pub fn input_message_content(&self) -> Option<Box<InputMessageContent>> { self.input_message_content.clone() }
  #[doc(hidden)] pub fn _set_input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



