
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A HTTP transparent proxy server. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyTypeHttp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // proxyTypeHttp
  /// Username for logging in; may be empty.
  username: Option<String>,
  /// Password for logging in; may be empty.
  password: Option<String>,
  /// Pass true, if the proxy supports only HTTP requests and doesn't support transparent TCP connections via HTTP CONNECT method.
  http_only: Option<bool>,
  
}



impl Object for ProxyTypeHttp {}
impl RObject for ProxyTypeHttp {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "proxyTypeHttp" }
  fn td_type(&self) -> RTDType { RTDType::ProxyTypeHttp }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ProxyType for ProxyTypeHttp {}


impl ProxyTypeHttp {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "proxyTypeHttp".to_string(),
      username: None,
      password: None,
      http_only: None,
      
    }
  }
  
  pub fn username(&self) -> Option<String> { self.username.clone() }
  #[doc(hidden)] pub fn _set_username(&mut self, username: String) -> &mut Self { self.username = Some(username); self }
  
  pub fn password(&self) -> Option<String> { self.password.clone() }
  #[doc(hidden)] pub fn _set_password(&mut self, password: String) -> &mut Self { self.password = Some(password); self }
  
  pub fn http_only(&self) -> Option<bool> { self.http_only.clone() }
  #[doc(hidden)] pub fn _set_http_only(&mut self, http_only: bool) -> &mut Self { self.http_only = Some(http_only); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



