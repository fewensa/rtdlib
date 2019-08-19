
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Searches for the specified query in the title and username of already known chats via request to the server. Returns chats in the order seen in the chat list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchChatsOnServer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchChatsOnServer
  /// Query to search for.
  query: Option<String>,
  /// Maximum number of chats to be returned.
  limit: Option<i32>,
  
}



impl Object for SearchChatsOnServer {}
impl RObject for SearchChatsOnServer {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchChatsOnServer" }
  fn td_type(&self) -> RTDType { RTDType::SearchChatsOnServer }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchChatsOnServer {}


impl SearchChatsOnServer {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchChatsOnServer".to_string(),
      query: None,
      limit: None,
      
    }
  }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



