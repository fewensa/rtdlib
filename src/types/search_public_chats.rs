
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Searches public chats by looking for specified query in their username and title. Currently only private chats, supergroups and channels can be public. Returns a meaningful number of results. Returns nothing if the length of the searched username prefix is less than 5. Excludes private chats with contacts and chats from the chat list from the results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPublicChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchPublicChats
  /// Query to search for.
  query: Option<String>,
  
}



impl Object for SearchPublicChats {}
impl RObject for SearchPublicChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchPublicChats" }
  fn td_type(&self) -> RTDType { RTDType::SearchPublicChats }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchPublicChats {}


impl SearchPublicChats {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchPublicChats".to_string(),
      query: None,
      
    }
  }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



