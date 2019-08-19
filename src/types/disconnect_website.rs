
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Disconnects website from the current user's Telegram account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisconnectWebsite {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // disconnectWebsite
  /// Website identifier.
  website_id: Option<i64>,
  
}



impl Object for DisconnectWebsite {}
impl RObject for DisconnectWebsite {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "disconnectWebsite" }
  fn td_type(&self) -> RTDType { RTDType::DisconnectWebsite }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DisconnectWebsite {}


impl DisconnectWebsite {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "disconnectWebsite".to_string(),
      website_id: None,
      
    }
  }
  
  pub fn website_id(&self) -> Option<i64> { self.website_id.clone() }
  #[doc(hidden)] pub fn _set_website_id(&mut self, website_id: i64) -> &mut Self { self.website_id = Some(website_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



