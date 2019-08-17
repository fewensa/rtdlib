
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is offline, but was online last week. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatusLastWeek {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userStatusLastWeek
  
}



impl Object for UserStatusLastWeek {}
impl RObject for UserStatusLastWeek {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userStatusLastWeek" }
  fn td_type(&self) -> RTDType { RTDType::UserStatusLastWeek }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserStatus for UserStatusLastWeek {}


impl UserStatusLastWeek {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userStatusLastWeek".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



