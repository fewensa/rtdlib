
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is offline, but was online last month. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatusLastMonth {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userStatusLastMonth
  
}



impl Object for UserStatusLastMonth {}
impl RObject for UserStatusLastMonth {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userStatusLastMonth" }
  fn td_type(&self) -> RTDType { RTDType::UserStatusLastMonth }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserStatus for UserStatusLastMonth {}


impl UserStatusLastMonth {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userStatusLastMonth".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



