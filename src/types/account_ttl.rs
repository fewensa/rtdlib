
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about the period of inactivity after which the current user's account will automatically be deleted. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountTtl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // accountTtl
  /// Number of days of inactivity before the account will be flagged for deletion; should range from 30-366 days.
  days: Option<i32>,
  
}



impl Object for AccountTtl {}
impl RObject for AccountTtl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "accountTtl" }
  fn td_type(&self) -> RTDType { RTDType::AccountTtl }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl AccountTtl {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "accountTtl".to_string(),
      days: None,
      
    }
  }
  
  pub fn days(&self) -> Option<i32> { self.days.clone() }
  #[doc(hidden)] pub fn _set_days(&mut self, days: i32) -> &mut Self { self.days = Some(days); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



