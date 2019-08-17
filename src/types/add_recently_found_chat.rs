
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds a chat to the list of recently found chats. The chat is added to the beginning of the list. If the chat is already in the list, it will be removed from the list first.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRecentlyFoundChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // addRecentlyFoundChat
  /// Identifier of the chat to add.
  chat_id: Option<i64>,
  
}



impl Object for AddRecentlyFoundChat {}
impl RObject for AddRecentlyFoundChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addRecentlyFoundChat" }
  fn td_type(&self) -> RTDType { RTDType::AddRecentlyFoundChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AddRecentlyFoundChat {}


impl AddRecentlyFoundChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "addRecentlyFoundChat".to_string(),
      chat_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



