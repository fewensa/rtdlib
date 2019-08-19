
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns users that were blocked by the current user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlockedUsers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getBlockedUsers
  /// Number of users to skip in the result; must be non-negative.
  offset: Option<i32>,
  /// Maximum number of users to return; up to 100.
  limit: Option<i32>,
  
}



impl Object for GetBlockedUsers {}
impl RObject for GetBlockedUsers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getBlockedUsers" }
  fn td_type(&self) -> RTDType { RTDType::GetBlockedUsers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetBlockedUsers {}


impl GetBlockedUsers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getBlockedUsers".to_string(),
      offset: None,
      limit: None,
      
    }
  }
  
  pub fn offset(&self) -> Option<i32> { self.offset.clone() }
  #[doc(hidden)] pub fn _set_offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



