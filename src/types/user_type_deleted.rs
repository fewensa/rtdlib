
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A deleted user or deleted bot. No information on the user besides the user_id is available. It is not possible to perform any active actions on this type of user. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTypeDeleted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userTypeDeleted
  
}



impl Object for UserTypeDeleted {}
impl RObject for UserTypeDeleted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userTypeDeleted" }
  fn td_type(&self) -> RTDType { RTDType::UserTypeDeleted }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserType for UserTypeDeleted {}


impl UserTypeDeleted {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userTypeDeleted".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



