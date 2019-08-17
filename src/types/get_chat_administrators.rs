
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a list of users who are administrators of the chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAdministrators {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getChatAdministrators
  /// Chat identifier.
  chat_id: Option<i64>,
  
}



impl Object for GetChatAdministrators {}
impl RObject for GetChatAdministrators {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatAdministrators" }
  fn td_type(&self) -> RTDType { RTDType::GetChatAdministrators }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetChatAdministrators {}


impl GetChatAdministrators {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getChatAdministrators".to_string(),
      chat_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



