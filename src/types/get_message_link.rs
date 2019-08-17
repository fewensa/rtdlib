
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a private HTTPS link to a message in a chat. Available only for already sent messages in supergroups and channels. The link will work only for members of the chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessageLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getMessageLink
  /// Identifier of the chat to which the message belongs.
  chat_id: Option<i64>,
  /// Identifier of the message.
  message_id: Option<i64>,
  
}



impl Object for GetMessageLink {}
impl RObject for GetMessageLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getMessageLink" }
  fn td_type(&self) -> RTDType { RTDType::GetMessageLink }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetMessageLink {}


impl GetMessageLink {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getMessageLink".to_string(),
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



