
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes the address of UDP reflectors. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallConnection {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callConnection
  /// Reflector identifier.
  id: Option<i64>,
  /// IPv4 reflector address.
  ip: Option<String>,
  /// IPv6 reflector address.
  ipv6: Option<String>,
  /// Reflector port number.
  port: Option<i32>,
  /// Connection peer tag.
  peer_tag: Option<String>,
  
}



impl Object for CallConnection {}
impl RObject for CallConnection {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callConnection" }
  fn td_type(&self) -> RTDType { RTDType::CallConnection }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl CallConnection {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callConnection".to_string(),
      id: None,
      ip: None,
      ipv6: None,
      port: None,
      peer_tag: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn ip(&self) -> Option<String> { self.ip.clone() }
  #[doc(hidden)] pub fn _set_ip(&mut self, ip: String) -> &mut Self { self.ip = Some(ip); self }
  
  pub fn ipv6(&self) -> Option<String> { self.ipv6.clone() }
  #[doc(hidden)] pub fn _set_ipv6(&mut self, ipv6: String) -> &mut Self { self.ipv6 = Some(ipv6); self }
  
  pub fn port(&self) -> Option<i32> { self.port.clone() }
  #[doc(hidden)] pub fn _set_port(&mut self, port: i32) -> &mut Self { self.port = Some(port); self }
  
  pub fn peer_tag(&self) -> Option<String> { self.peer_tag.clone() }
  #[doc(hidden)] pub fn _set_peer_tag(&mut self, peer_tag: String) -> &mut Self { self.peer_tag = Some(peer_tag); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



