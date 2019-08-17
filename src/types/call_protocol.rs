
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Specifies the supported call protocols. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallProtocol {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callProtocol
  /// True, if UDP peer-to-peer connections are supported.
  udp_p2p: Option<bool>,
  /// True, if connection through UDP reflectors is supported.
  udp_reflector: Option<bool>,
  /// Minimum supported API layer; use 65.
  min_layer: Option<i32>,
  /// Maximum supported API layer; use 65.
  max_layer: Option<i32>,
  
}



impl Object for CallProtocol {}
impl RObject for CallProtocol {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callProtocol" }
  fn td_type(&self) -> RTDType { RTDType::CallProtocol }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl CallProtocol {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callProtocol".to_string(),
      udp_p2p: None,
      udp_reflector: None,
      min_layer: None,
      max_layer: None,
      
    }
  }
  
  pub fn udp_p2p(&self) -> Option<bool> { self.udp_p2p.clone() }
  #[doc(hidden)] pub fn _set_udp_p2p(&mut self, udp_p2p: bool) -> &mut Self { self.udp_p2p = Some(udp_p2p); self }
  
  pub fn udp_reflector(&self) -> Option<bool> { self.udp_reflector.clone() }
  #[doc(hidden)] pub fn _set_udp_reflector(&mut self, udp_reflector: bool) -> &mut Self { self.udp_reflector = Some(udp_reflector); self }
  
  pub fn min_layer(&self) -> Option<i32> { self.min_layer.clone() }
  #[doc(hidden)] pub fn _set_min_layer(&mut self, min_layer: i32) -> &mut Self { self.min_layer = Some(min_layer); self }
  
  pub fn max_layer(&self) -> Option<i32> { self.max_layer.clone() }
  #[doc(hidden)] pub fn _set_max_layer(&mut self, max_layer: i32) -> &mut Self { self.max_layer = Some(max_layer); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



