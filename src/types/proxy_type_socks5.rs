
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A SOCKS5 proxy server. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyTypeSocks5 {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // proxyTypeSocks5
  /// Username for logging in; may be empty.
  username: Option<String>,
  /// Password for logging in; may be empty.
  password: Option<String>,
  
}



impl Object for ProxyTypeSocks5 {}
impl RObject for ProxyTypeSocks5 {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "proxyTypeSocks5" }
  fn td_type(&self) -> RTDType { RTDType::ProxyTypeSocks5 }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ProxyType for ProxyTypeSocks5 {}


impl ProxyTypeSocks5 {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "proxyTypeSocks5".to_string(),
      username: None,
      password: None,
      
    }
  }
  
  pub fn username(&self) -> Option<String> { self.username.clone() }
  #[doc(hidden)] pub fn _set_username(&mut self, username: String) -> &mut Self { self.username = Some(username); self }
  
  pub fn password(&self) -> Option<String> { self.password.clone() }
  #[doc(hidden)] pub fn _set_password(&mut self, password: String) -> &mut Self { self.password = Some(password); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



