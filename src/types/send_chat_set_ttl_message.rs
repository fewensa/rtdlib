
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the current TTL setting (sets a new self-destruct timer) in a secret chat and sends the corresponding message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendChatSetTtlMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sendChatSetTtlMessage
  /// Chat identifier.
  chat_id: Option<i64>,
  /// New TTL value, in seconds.
  ttl: Option<i32>,
  
}



impl Object for SendChatSetTtlMessage {}
impl RObject for SendChatSetTtlMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendChatSetTtlMessage" }
  fn td_type(&self) -> RTDType { RTDType::SendChatSetTtlMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SendChatSetTtlMessage {}


impl SendChatSetTtlMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sendChatSetTtlMessage".to_string(),
      chat_id: None,
      ttl: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn ttl(&self) -> Option<i32> { self.ttl.clone() }
  #[doc(hidden)] pub fn _set_ttl(&mut self, ttl: i32) -> &mut Self { self.ttl = Some(ttl); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



