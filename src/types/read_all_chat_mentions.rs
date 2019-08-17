
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Marks all mentions in a chat as read.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadAllChatMentions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // readAllChatMentions
  /// Chat identifier.
  chat_id: Option<i64>,
  
}



impl Object for ReadAllChatMentions {}
impl RObject for ReadAllChatMentions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "readAllChatMentions" }
  fn td_type(&self) -> RTDType { RTDType::ReadAllChatMentions }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ReadAllChatMentions {}


impl ReadAllChatMentions {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "readAllChatMentions".to_string(),
      chat_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



