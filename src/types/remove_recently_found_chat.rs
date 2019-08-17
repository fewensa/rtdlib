
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Removes a chat from the list of recently found chats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveRecentlyFoundChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // removeRecentlyFoundChat
  /// Identifier of the chat to be removed.
  chat_id: Option<i64>,
  
}



impl Object for RemoveRecentlyFoundChat {}
impl RObject for RemoveRecentlyFoundChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeRecentlyFoundChat" }
  fn td_type(&self) -> RTDType { RTDType::RemoveRecentlyFoundChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RemoveRecentlyFoundChat {}


impl RemoveRecentlyFoundChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "removeRecentlyFoundChat".to_string(),
      chat_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



