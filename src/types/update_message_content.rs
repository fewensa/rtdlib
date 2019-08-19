
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The message content has changed. 
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMessageContent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateMessageContent
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Message identifier.
  message_id: Option<i64>,
  /// New message content.
  new_content: Option<Box<MessageContent>>,
  
}


impl Clone for UpdateMessageContent {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UpdateMessageContent {}
impl RObject for UpdateMessageContent {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageContent" }
  fn td_type(&self) -> RTDType { RTDType::UpdateMessageContent }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateMessageContent {}


impl UpdateMessageContent {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateMessageContent".to_string(),
      chat_id: None,
      message_id: None,
      new_content: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn new_content(&self) -> Option<Box<MessageContent>> { self.new_content.clone() }
  #[doc(hidden)] pub fn _set_new_content(&mut self, new_content: Box<MessageContent>) -> &mut Self { self.new_content = Some(new_content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



