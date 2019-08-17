
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a list of frequently used chats. Supported only if the chat info database is enabled.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTopChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getTopChats
  /// Category of chats to be returned.
  category: Option<Box<TopChatCategory>>,
  /// Maximum number of chats to be returned; up to 30.
  limit: Option<i32>,
  
}


impl Clone for GetTopChats {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for GetTopChats {}
impl RObject for GetTopChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getTopChats" }
  fn td_type(&self) -> RTDType { RTDType::GetTopChats }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetTopChats {}


impl GetTopChats {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getTopChats".to_string(),
      category: None,
      limit: None,
      
    }
  }
  
  pub fn category(&self) -> Option<Box<TopChatCategory>> { self.category.clone() }
  #[doc(hidden)] pub fn _set_category(&mut self, category: Box<TopChatCategory>) -> &mut Self { self.category = Some(category); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



