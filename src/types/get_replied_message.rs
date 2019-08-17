
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about a message that is replied by given message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRepliedMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getRepliedMessage
  /// Identifier of the chat the message belongs to.
  chat_id: Option<i64>,
  /// Identifier of the message reply to which get.
  message_id: Option<i64>,
  
}



impl Object for GetRepliedMessage {}
impl RObject for GetRepliedMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getRepliedMessage" }
  fn td_type(&self) -> RTDType { RTDType::GetRepliedMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetRepliedMessage {}


impl GetRepliedMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getRepliedMessage".to_string(),
      chat_id: None,
      message_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



