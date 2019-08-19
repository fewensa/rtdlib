
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds a proxy server for network requests. Can be called before authorization.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddProxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // addProxy
  /// Proxy server IP address.
  server: Option<String>,
  /// Proxy server port.
  port: Option<i32>,
  /// True, if the proxy should be enabled.
  enable: Option<bool>,
  /// Proxy type.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<ProxyType>>,
  
}


impl Clone for AddProxy {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for AddProxy {}
impl RObject for AddProxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addProxy" }
  fn td_type(&self) -> RTDType { RTDType::AddProxy }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AddProxy {}


impl AddProxy {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "addProxy".to_string(),
      server: None,
      port: None,
      enable: None,
      type_: None,
      
    }
  }
  
  pub fn server(&self) -> Option<String> { self.server.clone() }
  #[doc(hidden)] pub fn _set_server(&mut self, server: String) -> &mut Self { self.server = Some(server); self }
  
  pub fn port(&self) -> Option<i32> { self.port.clone() }
  #[doc(hidden)] pub fn _set_port(&mut self, port: i32) -> &mut Self { self.port = Some(port); self }
  
  pub fn enable(&self) -> Option<bool> { self.enable.clone() }
  #[doc(hidden)] pub fn _set_enable(&mut self, enable: bool) -> &mut Self { self.enable = Some(enable); self }
  
  pub fn type_(&self) -> Option<Box<ProxyType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<ProxyType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



