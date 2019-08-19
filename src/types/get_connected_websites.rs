
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns all website where the current user used Telegram to log in.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConnectedWebsites {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getConnectedWebsites
  
}



impl Object for GetConnectedWebsites {}
impl RObject for GetConnectedWebsites {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getConnectedWebsites" }
  fn td_type(&self) -> RTDType { RTDType::GetConnectedWebsites }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetConnectedWebsites {}


impl GetConnectedWebsites {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getConnectedWebsites".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



