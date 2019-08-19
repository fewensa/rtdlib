
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Deletes saved credentials for all payment provider bots.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSavedCredentials {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deleteSavedCredentials
  
}



impl Object for DeleteSavedCredentials {}
impl RObject for DeleteSavedCredentials {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteSavedCredentials" }
  fn td_type(&self) -> RTDType { RTDType::DeleteSavedCredentials }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DeleteSavedCredentials {}


impl DeleteSavedCredentials {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deleteSavedCredentials".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



