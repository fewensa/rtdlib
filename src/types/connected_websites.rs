
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a list of websites the current user is logged in with Telegram. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedWebsites {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // connectedWebsites
  /// List of connected websites.
  websites: Option<Vec<ConnectedWebsite>>,
  
}



impl Object for ConnectedWebsites {}
impl RObject for ConnectedWebsites {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectedWebsites" }
  fn td_type(&self) -> RTDType { RTDType::ConnectedWebsites }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ConnectedWebsites {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "connectedWebsites".to_string(),
      websites: None,
      
    }
  }
  
  pub fn websites(&self) -> Option<Vec<ConnectedWebsite>> { self.websites.clone() }
  #[doc(hidden)] pub fn _set_websites(&mut self, websites: Vec<ConnectedWebsite>) -> &mut Self { self.websites = Some(websites); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



