
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the order of pinned chats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPinnedChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setPinnedChats
  /// The new list of pinned chats.
  chat_ids: Option<Vec<i64>>,
  
}



impl Object for SetPinnedChats {}
impl RObject for SetPinnedChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setPinnedChats" }
  fn td_type(&self) -> RTDType { RTDType::SetPinnedChats }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetPinnedChats {}


impl SetPinnedChats {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setPinnedChats".to_string(),
      chat_ids: None,
      
    }
  }
  
  pub fn chat_ids(&self) -> Option<Vec<i64>> { self.chat_ids.clone() }
  #[doc(hidden)] pub fn _set_chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self { self.chat_ids = Some(chat_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



