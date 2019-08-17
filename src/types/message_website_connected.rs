
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The current user has connected a website by logging in using Telegram Login Widget on it. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageWebsiteConnected {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageWebsiteConnected
  /// Domain name of the connected website.
  domain_name: Option<String>,
  
}



impl Object for MessageWebsiteConnected {}
impl RObject for MessageWebsiteConnected {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageWebsiteConnected" }
  fn td_type(&self) -> RTDType { RTDType::MessageWebsiteConnected }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageWebsiteConnected {}


impl MessageWebsiteConnected {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageWebsiteConnected".to_string(),
      domain_name: None,
      
    }
  }
  
  pub fn domain_name(&self) -> Option<String> { self.domain_name.clone() }
  #[doc(hidden)] pub fn _set_domain_name(&mut self, domain_name: String) -> &mut Self { self.domain_name = Some(domain_name); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



