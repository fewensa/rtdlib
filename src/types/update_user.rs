
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the client. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateUser
  /// New data about the user.
  user: Option<User>,
  
}



impl Object for UpdateUser {}
impl RObject for UpdateUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateUser" }
  fn td_type(&self) -> RTDType { RTDType::UpdateUser }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateUser {}


impl UpdateUser {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateUser".to_string(),
      user: None,
      
    }
  }
  
  pub fn user(&self) -> Option<User> { self.user.clone() }
  #[doc(hidden)] pub fn _set_user(&mut self, user: User) -> &mut Self { self.user = Some(user); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



