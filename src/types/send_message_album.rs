
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends messages grouped together into an album. Currently only photo and video messages can be grouped into an album. Returns sent messages.
#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageAlbum {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sendMessageAlbum
  /// Target chat.
  chat_id: Option<i64>,
  /// Identifier of a message to reply to or 0.
  reply_to_message_id: Option<i64>,
  /// Pass true to disable notification for the messages. Not supported in secret chats.
  disable_notification: Option<bool>,
  /// Pass true if the messages are sent from the background.
  from_background: Option<bool>,
  /// Contents of messages to be sent.
  input_message_contents: Option<Vec<Box<InputMessageContent>>>,
  
}


impl Clone for SendMessageAlbum {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SendMessageAlbum {}
impl RObject for SendMessageAlbum {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendMessageAlbum" }
  fn td_type(&self) -> RTDType { RTDType::SendMessageAlbum }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SendMessageAlbum {}


impl SendMessageAlbum {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sendMessageAlbum".to_string(),
      chat_id: None,
      reply_to_message_id: None,
      disable_notification: None,
      from_background: None,
      input_message_contents: None,
      
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
  
  pub fn input_message_contents(&self) -> Option<Vec<Box<InputMessageContent>>> { self.input_message_contents.clone() }
  #[doc(hidden)] pub fn _set_input_message_contents(&mut self, input_message_contents: Vec<Box<InputMessageContent>>) -> &mut Self { self.input_message_contents = Some(input_message_contents); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



