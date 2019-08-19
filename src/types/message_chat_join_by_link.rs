
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new member joined the chat by invite link. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageChatJoinByLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageChatJoinByLink
  
}



impl Object for MessageChatJoinByLink {}
impl RObject for MessageChatJoinByLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatJoinByLink" }
  fn td_type(&self) -> RTDType { RTDType::MessageChatJoinByLink }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageChatJoinByLink {}


impl MessageChatJoinByLink {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageChatJoinByLink".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



