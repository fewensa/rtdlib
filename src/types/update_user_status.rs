
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user went online or offline. 
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateUserStatus
  /// User identifier.
  user_id: Option<i32>,
  /// New status of the user.
  status: Option<Box<UserStatus>>,
  
}


impl Clone for UpdateUserStatus {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UpdateUserStatus {}
impl RObject for UpdateUserStatus {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateUserStatus" }
  fn td_type(&self) -> RTDType { RTDType::UpdateUserStatus }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateUserStatus {}


impl UpdateUserStatus {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateUserStatus".to_string(),
      user_id: None,
      status: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn status(&self) -> Option<Box<UserStatus>> { self.status.clone() }
  #[doc(hidden)] pub fn _set_status(&mut self, status: Box<UserStatus>) -> &mut Self { self.status = Some(status); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



