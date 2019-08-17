
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about a user by their identifier. This is an offline request if the current user is not a bot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getUser
  /// User identifier.
  user_id: Option<i32>,
  
}



impl Object for GetUser {}
impl RObject for GetUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getUser" }
  fn td_type(&self) -> RTDType { RTDType::GetUser }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetUser {}


impl GetUser {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getUser".to_string(),
      user_id: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



