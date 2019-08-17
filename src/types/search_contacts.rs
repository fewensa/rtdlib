
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Searches for the specified query in the first names, last names and usernames of the known user contacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchContacts
  /// Query to search for; may be empty to return all contacts.
  query: Option<String>,
  /// Maximum number of users to be returned.
  limit: Option<i32>,
  
}



impl Object for SearchContacts {}
impl RObject for SearchContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchContacts" }
  fn td_type(&self) -> RTDType { RTDType::SearchContacts }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchContacts {}


impl SearchContacts {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchContacts".to_string(),
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



