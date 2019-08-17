
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns approximate number of messages of the specified type in the chat.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetChatMessageCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getChatMessageCount
  /// Identifier of the chat in which to count messages.
  chat_id: Option<i64>,
  /// Filter for message content; searchMessagesFilterEmpty is unsupported in this function.
  filter: Option<Box<SearchMessagesFilter>>,
  /// If true, returns count that is available locally without sending network requests, returning -1 if the number of messages is unknown.
  return_local: Option<bool>,
  
}


impl Clone for GetChatMessageCount {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for GetChatMessageCount {}
impl RObject for GetChatMessageCount {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatMessageCount" }
  fn td_type(&self) -> RTDType { RTDType::GetChatMessageCount }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetChatMessageCount {}


impl GetChatMessageCount {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getChatMessageCount".to_string(),
      chat_id: None,
      filter: None,
      return_local: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn filter(&self) -> Option<Box<SearchMessagesFilter>> { self.filter.clone() }
  #[doc(hidden)] pub fn _set_filter(&mut self, filter: Box<SearchMessagesFilter>) -> &mut Self { self.filter = Some(filter); self }
  
  pub fn return_local(&self) -> Option<bool> { self.return_local.clone() }
  #[doc(hidden)] pub fn _set_return_local(&mut self, return_local: bool) -> &mut Self { self.return_local = Some(return_local); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



