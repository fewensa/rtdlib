
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is offline. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatusOffline {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userStatusOffline
  /// Point in time (Unix timestamp) when the user was last online.
  was_online: Option<i32>,
  
}



impl Object for UserStatusOffline {}
impl RObject for UserStatusOffline {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userStatusOffline" }
  fn td_type(&self) -> RTDType { RTDType::UserStatusOffline }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserStatus for UserStatusOffline {}


impl UserStatusOffline {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userStatusOffline".to_string(),
      was_online: None,
      
    }
  }
  
  pub fn was_online(&self) -> Option<i32> { self.was_online.clone() }
  #[doc(hidden)] pub fn _set_was_online(&mut self, was_online: i32) -> &mut Self { self.was_online = Some(was_online); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



