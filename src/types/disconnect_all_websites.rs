
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Disconnects all websites from the current user's Telegram account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisconnectAllWebsites {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // disconnectAllWebsites
  
}



impl Object for DisconnectAllWebsites {}
impl RObject for DisconnectAllWebsites {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "disconnectAllWebsites" }
  fn td_type(&self) -> RTDType { RTDType::DisconnectAllWebsites }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DisconnectAllWebsites {}


impl DisconnectAllWebsites {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "disconnectAllWebsites".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



