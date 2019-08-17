
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A URL linking to a user. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TMeUrlTypeUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // tMeUrlTypeUser
  /// Identifier of the user.
  user_id: Option<i32>,
  
}



impl Object for TMeUrlTypeUser {}
impl RObject for TMeUrlTypeUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tMeUrlTypeUser" }
  fn td_type(&self) -> RTDType { RTDType::TMeUrlTypeUser }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TMeUrlType for TMeUrlTypeUser {}


impl TMeUrlTypeUser {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "tMeUrlTypeUser".to_string(),
      user_id: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



