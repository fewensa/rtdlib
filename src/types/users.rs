
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a list of users. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Users {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // users
  /// Approximate total count of users found.
  total_count: Option<i32>,
  /// A list of user identifiers.
  user_ids: Option<Vec<i32>>,
  
}



impl Object for Users {}
impl RObject for Users {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "users" }
  fn td_type(&self) -> RTDType { RTDType::Users }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Users {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "users".to_string(),
      total_count: None,
      user_ids: None,
      
    }
  }
  
  pub fn total_count(&self) -> Option<i32> { self.total_count.clone() }
  #[doc(hidden)] pub fn _set_total_count(&mut self, total_count: i32) -> &mut Self { self.total_count = Some(total_count); self }
  
  pub fn user_ids(&self) -> Option<Vec<i32>> { self.user_ids.clone() }
  #[doc(hidden)] pub fn _set_user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self { self.user_ids = Some(user_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



