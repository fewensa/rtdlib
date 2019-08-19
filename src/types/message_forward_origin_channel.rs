
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The message was originally a post in a channel. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageForwardOriginChannel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageForwardOriginChannel
  /// Identifier of the chat from which the message was originally forwarded.
  chat_id: Option<i64>,
  /// Message identifier of the original message; 0 if unknown.
  message_id: Option<i64>,
  /// Original post author signature.
  author_signature: Option<String>,
  
}



impl Object for MessageForwardOriginChannel {}
impl RObject for MessageForwardOriginChannel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageForwardOriginChannel" }
  fn td_type(&self) -> RTDType { RTDType::MessageForwardOriginChannel }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageForwardOrigin for MessageForwardOriginChannel {}


impl MessageForwardOriginChannel {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageForwardOriginChannel".to_string(),
      chat_id: None,
      message_id: None,
      author_signature: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn author_signature(&self) -> Option<String> { self.author_signature.clone() }
  #[doc(hidden)] pub fn _set_author_signature(&mut self, author_signature: String) -> &mut Self { self.author_signature = Some(author_signature); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



