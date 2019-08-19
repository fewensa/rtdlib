
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a list of common group chats with a given user. Chats are sorted by their type and creation date.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGroupsInCommon {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getGroupsInCommon
  /// User identifier.
  user_id: Option<i32>,
  /// Chat identifier starting from which to return chats; use 0 for the first request.
  offset_chat_id: Option<i64>,
  /// Maximum number of chats to be returned; up to 100.
  limit: Option<i32>,
  
}



impl Object for GetGroupsInCommon {}
impl RObject for GetGroupsInCommon {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getGroupsInCommon" }
  fn td_type(&self) -> RTDType { RTDType::GetGroupsInCommon }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetGroupsInCommon {}


impl GetGroupsInCommon {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getGroupsInCommon".to_string(),
      user_id: None,
      offset_chat_id: None,
      limit: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn offset_chat_id(&self) -> Option<i64> { self.offset_chat_id.clone() }
  #[doc(hidden)] pub fn _set_offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self { self.offset_chat_id = Some(offset_chat_id); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



