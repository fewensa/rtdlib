
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Forwards previously sent messages. Returns the forwarded messages in the same order as the message identifiers passed in message_ids. If a message can't be forwarded, null will be returned instead of the message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // forwardMessages
  /// Identifier of the chat to which to forward messages.
  chat_id: Option<i64>,
  /// Identifier of the chat from which to forward messages.
  from_chat_id: Option<i64>,
  /// Identifiers of the messages to forward.
  message_ids: Option<Vec<i64>>,
  /// Pass true to disable notification for the message, doesn't work if messages are forwarded to a secret chat.
  disable_notification: Option<bool>,
  /// Pass true if the message is sent from the background.
  from_background: Option<bool>,
  /// True, if the messages should be grouped into an album after forwarding. For this to work, no more than 10 messages may be forwarded, and all of them must be photo or video messages.
  as_album: Option<bool>,
  
}



impl Object for ForwardMessages {}
impl RObject for ForwardMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "forwardMessages" }
  fn td_type(&self) -> RTDType { RTDType::ForwardMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ForwardMessages {}


impl ForwardMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "forwardMessages".to_string(),
      chat_id: None,
      from_chat_id: None,
      message_ids: None,
      disable_notification: None,
      from_background: None,
      as_album: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_chat_id(&self) -> Option<i64> { self.from_chat_id.clone() }
  #[doc(hidden)] pub fn _set_from_chat_id(&mut self, from_chat_id: i64) -> &mut Self { self.from_chat_id = Some(from_chat_id); self }
  
  pub fn message_ids(&self) -> Option<Vec<i64>> { self.message_ids.clone() }
  #[doc(hidden)] pub fn _set_message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self { self.message_ids = Some(message_ids); self }
  
  pub fn disable_notification(&self) -> Option<bool> { self.disable_notification.clone() }
  #[doc(hidden)] pub fn _set_disable_notification(&mut self, disable_notification: bool) -> &mut Self { self.disable_notification = Some(disable_notification); self }
  
  pub fn from_background(&self) -> Option<bool> { self.from_background.clone() }
  #[doc(hidden)] pub fn _set_from_background(&mut self, from_background: bool) -> &mut Self { self.from_background = Some(from_background); self }
  
  pub fn as_album(&self) -> Option<bool> { self.as_album.clone() }
  #[doc(hidden)] pub fn _set_as_album(&mut self, as_album: bool) -> &mut Self { self.as_album = Some(as_album); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



