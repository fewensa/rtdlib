
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is online. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatusOnline {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userStatusOnline
  /// Point in time (Unix timestamp) when the user's online status will expire.
  expires: Option<i32>,
  
}



impl Object for UserStatusOnline {}
impl RObject for UserStatusOnline {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userStatusOnline" }
  fn td_type(&self) -> RTDType { RTDType::UserStatusOnline }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserStatus for UserStatusOnline {}


impl UserStatusOnline {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userStatusOnline".to_string(),
      expires: None,
      
    }
  }
  
  pub fn expires(&self) -> Option<i32> { self.expires.clone() }
  #[doc(hidden)] pub fn _set_expires(&mut self, expires: i32) -> &mut Self { self.expires = Some(expires); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



