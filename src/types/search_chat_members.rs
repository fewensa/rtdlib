
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Searches for a specified query in the first name, last name and username of the members of a specified chat. Requires administrator rights in channels.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchChatMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchChatMembers
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Query to search for.
  query: Option<String>,
  /// The maximum number of users to be returned.
  limit: Option<i32>,
  /// The type of users to return. By default, chatMembersFilterMembers.
  filter: Option<Box<ChatMembersFilter>>,
  
}


impl Clone for SearchChatMembers {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SearchChatMembers {}
impl RObject for SearchChatMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchChatMembers" }
  fn td_type(&self) -> RTDType { RTDType::SearchChatMembers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchChatMembers {}


impl SearchChatMembers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchChatMembers".to_string(),
      chat_id: None,
      query: None,
      limit: None,
      filter: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn filter(&self) -> Option<Box<ChatMembersFilter>> { self.filter.clone() }
  #[doc(hidden)] pub fn _set_filter(&mut self, filter: Box<ChatMembersFilter>) -> &mut Self { self.filter = Some(filter); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



