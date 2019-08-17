
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A regular user. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTypeRegular {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userTypeRegular
  
}



impl Object for UserTypeRegular {}
impl RObject for UserTypeRegular {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userTypeRegular" }
  fn td_type(&self) -> RTDType { RTDType::UserTypeRegular }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserType for UserTypeRegular {}


impl UserTypeRegular {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userTypeRegular".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



