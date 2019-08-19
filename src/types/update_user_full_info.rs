
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Some data from 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateUserFullInfo
  /// User identifier.
  user_id: Option<i32>,
  /// New full information about the user.
  user_full_info: Option<UserFullInfo>,
  
}



impl Object for UpdateUserFullInfo {}
impl RObject for UpdateUserFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateUserFullInfo" }
  fn td_type(&self) -> RTDType { RTDType::UpdateUserFullInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateUserFullInfo {}


impl UpdateUserFullInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateUserFullInfo".to_string(),
      user_id: None,
      user_full_info: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn user_full_info(&self) -> Option<UserFullInfo> { self.user_full_info.clone() }
  #[doc(hidden)] pub fn _set_user_full_info(&mut self, user_full_info: UserFullInfo) -> &mut Self { self.user_full_info = Some(user_full_info); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



