
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Informs TDLib that the message content has been opened (e.g., the user has opened a photo, video, document, location or venue, or has listened to an audio file or voice note message). An 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMessageContent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // openMessageContent
  /// Chat identifier of the message.
  chat_id: Option<i64>,
  /// Identifier of the message with the opened content.
  message_id: Option<i64>,
  
}



impl Object for OpenMessageContent {}
impl RObject for OpenMessageContent {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "openMessageContent" }
  fn td_type(&self) -> RTDType { RTDType::OpenMessageContent }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for OpenMessageContent {}


impl OpenMessageContent {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "openMessageContent".to_string(),
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



