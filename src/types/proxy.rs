
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about a proxy server. 
#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // proxy
  /// Unique identifier of the proxy.
  id: Option<i32>,
  /// Proxy server IP address.
  server: Option<String>,
  /// Proxy server port.
  port: Option<i32>,
  /// Point in time (Unix timestamp) when the proxy was last used; 0 if never.
  last_used_date: Option<i32>,
  /// True, if the proxy is enabled now.
  is_enabled: Option<bool>,
  /// Type of the proxy.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<ProxyType>>,
  
}


impl Clone for Proxy {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for Proxy {}
impl RObject for Proxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "proxy" }
  fn td_type(&self) -> RTDType { RTDType::Proxy }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Proxy {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "proxy".to_string(),
      id: None,
      server: None,
      port: None,
      last_used_date: None,
      is_enabled: None,
      type_: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i32> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i32) -> &mut Self { self.id = Some(id); self }
  
  pub fn server(&self) -> Option<String> { self.server.clone() }
  #[doc(hidden)] pub fn _set_server(&mut self, server: String) -> &mut Self { self.server = Some(server); self }
  
  pub fn port(&self) -> Option<i32> { self.port.clone() }
  #[doc(hidden)] pub fn _set_port(&mut self, port: i32) -> &mut Self { self.port = Some(port); self }
  
  pub fn last_used_date(&self) -> Option<i32> { self.last_used_date.clone() }
  #[doc(hidden)] pub fn _set_last_used_date(&mut self, last_used_date: i32) -> &mut Self { self.last_used_date = Some(last_used_date); self }
  
  pub fn is_enabled(&self) -> Option<bool> { self.is_enabled.clone() }
  #[doc(hidden)] pub fn _set_is_enabled(&mut self, is_enabled: bool) -> &mut Self { self.is_enabled = Some(is_enabled); self }
  
  pub fn type_(&self) -> Option<Box<ProxyType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<ProxyType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



