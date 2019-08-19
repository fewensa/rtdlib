
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user was online recently. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatusRecently {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userStatusRecently
  
}



impl Object for UserStatusRecently {}
impl RObject for UserStatusRecently {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userStatusRecently" }
  fn td_type(&self) -> RTDType { RTDType::UserStatusRecently }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserStatus for UserStatusRecently {}


impl UserStatusRecently {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userStatusRecently".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



