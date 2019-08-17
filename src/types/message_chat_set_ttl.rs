
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The TTL (Time To Live) setting messages in a secret chat has been changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageChatSetTtl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageChatSetTtl
  /// New TTL.
  ttl: Option<i32>,
  
}



impl Object for MessageChatSetTtl {}
impl RObject for MessageChatSetTtl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatSetTtl" }
  fn td_type(&self) -> RTDType { RTDType::MessageChatSetTtl }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageChatSetTtl {}


impl MessageChatSetTtl {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageChatSetTtl".to_string(),
      ttl: None,
      
    }
  }
  
  pub fn ttl(&self) -> Option<i32> { self.ttl.clone() }
  #[doc(hidden)] pub fn _set_ttl(&mut self, ttl: i32) -> &mut Self { self.ttl = Some(ttl); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



