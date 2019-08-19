
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a user that can be contacted to get support.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSupportUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getSupportUser
  
}



impl Object for GetSupportUser {}
impl RObject for GetSupportUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getSupportUser" }
  fn td_type(&self) -> RTDType { RTDType::GetSupportUser }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetSupportUser {}


impl GetSupportUser {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getSupportUser".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



