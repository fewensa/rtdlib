
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// No information on the user besides the user_id is available, yet this user has not been deleted. This object is extremely rare and must be handled like a deleted user. It is not possible to perform any actions on users of this type. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTypeUnknown {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userTypeUnknown
  
}



impl Object for UserTypeUnknown {}
impl RObject for UserTypeUnknown {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userTypeUnknown" }
  fn td_type(&self) -> RTDType { RTDType::UserTypeUnknown }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserType for UserTypeUnknown {}


impl UserTypeUnknown {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userTypeUnknown".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



