
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a list of proxy servers. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxies {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // proxies
  /// List of proxy servers.
  proxies: Option<Vec<Proxy>>,
  
}



impl Object for Proxies {}
impl RObject for Proxies {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "proxies" }
  fn td_type(&self) -> RTDType { RTDType::Proxies }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Proxies {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "proxies".to_string(),
      proxies: None,
      
    }
  }
  
  pub fn proxies(&self) -> Option<Vec<Proxy>> { self.proxies.clone() }
  #[doc(hidden)] pub fn _set_proxies(&mut self, proxies: Vec<Proxy>) -> &mut Self { self.proxies = Some(proxies); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



