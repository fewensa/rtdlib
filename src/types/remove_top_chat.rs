
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Removes a chat from the list of frequently used chats. Supported only if the chat info database is enabled.
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveTopChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // removeTopChat
  /// Category of frequently used chats.
  category: Option<Box<TopChatCategory>>,
  /// Chat identifier.
  chat_id: Option<i64>,
  
}


impl Clone for RemoveTopChat {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RemoveTopChat {}
impl RObject for RemoveTopChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeTopChat" }
  fn td_type(&self) -> RTDType { RTDType::RemoveTopChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RemoveTopChat {}


impl RemoveTopChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "removeTopChat".to_string(),
      category: None,
      chat_id: None,
      
    }
  }
  
  pub fn category(&self) -> Option<Box<TopChatCategory>> { self.category.clone() }
  #[doc(hidden)] pub fn _set_category(&mut self, category: Box<TopChatCategory>) -> &mut Self { self.category = Some(category); self }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



