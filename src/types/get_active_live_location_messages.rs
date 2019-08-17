
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns all active live locations that should be updated by the client. The list is persistent across application restarts only if the message database is used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActiveLiveLocationMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getActiveLiveLocationMessages
  
}



impl Object for GetActiveLiveLocationMessages {}
impl RObject for GetActiveLiveLocationMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getActiveLiveLocationMessages" }
  fn td_type(&self) -> RTDType { RTDType::GetActiveLiveLocationMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetActiveLiveLocationMessages {}


impl GetActiveLiveLocationMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getActiveLiveLocationMessages".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



