
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about a forwarded message. 
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageForwardInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageForwardInfo
  /// Origin of a forwarded message.
  origin: Option<Box<MessageForwardOrigin>>,
  /// Point in time (Unix timestamp) when the message was originally sent.
  date: Option<i32>,
  /// For messages forwarded to the chat with the current user (saved messages), the identifier of the chat from which the message was forwarded last time; 0 if unknown.
  from_chat_id: Option<i64>,
  /// For messages forwarded to the chat with the current user (saved messages), the identifier of the original message from which the new message was forwarded last time; 0 if unknown.
  from_message_id: Option<i64>,
  
}


impl Clone for MessageForwardInfo {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for MessageForwardInfo {}
impl RObject for MessageForwardInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageForwardInfo" }
  fn td_type(&self) -> RTDType { RTDType::MessageForwardInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl MessageForwardInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageForwardInfo".to_string(),
      origin: None,
      date: None,
      from_chat_id: None,
      from_message_id: None,
      
    }
  }
  
  pub fn origin(&self) -> Option<Box<MessageForwardOrigin>> { self.origin.clone() }
  #[doc(hidden)] pub fn _set_origin(&mut self, origin: Box<MessageForwardOrigin>) -> &mut Self { self.origin = Some(origin); self }
  
  pub fn date(&self) -> Option<i32> { self.date.clone() }
  #[doc(hidden)] pub fn _set_date(&mut self, date: i32) -> &mut Self { self.date = Some(date); self }
  
  pub fn from_chat_id(&self) -> Option<i64> { self.from_chat_id.clone() }
  #[doc(hidden)] pub fn _set_from_chat_id(&mut self, from_chat_id: i64) -> &mut Self { self.from_chat_id = Some(from_chat_id); self }
  
  pub fn from_message_id(&self) -> Option<i64> { self.from_message_id.clone() }
  #[doc(hidden)] pub fn _set_from_message_id(&mut self, from_message_id: i64) -> &mut Self { self.from_message_id = Some(from_message_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



