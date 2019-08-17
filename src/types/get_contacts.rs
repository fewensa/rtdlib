
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns all user contacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getContacts
  
}



impl Object for GetContacts {}
impl RObject for GetContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getContacts" }
  fn td_type(&self) -> RTDType { RTDType::GetContacts }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetContacts {}


impl GetContacts {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getContacts".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



