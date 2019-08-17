
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The call is ready to use. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStateReady {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callStateReady
  /// Call protocols supported by the peer.
  protocol: Option<CallProtocol>,
  /// Available UDP reflectors.
  connections: Option<Vec<CallConnection>>,
  /// A JSON-encoded call config.
  config: Option<String>,
  /// Call encryption key.
  encryption_key: Option<String>,
  /// Encryption key emojis fingerprint.
  emojis: Option<Vec<String>>,
  /// True, if peer-to-peer connection is allowed by users privacy settings.
  allow_p2p: Option<bool>,
  
}



impl Object for CallStateReady {}
impl RObject for CallStateReady {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callStateReady" }
  fn td_type(&self) -> RTDType { RTDType::CallStateReady }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CallState for CallStateReady {}


impl CallStateReady {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callStateReady".to_string(),
      protocol: None,
      connections: None,
      config: None,
      encryption_key: None,
      emojis: None,
      allow_p2p: None,
      
    }
  }
  
  pub fn protocol(&self) -> Option<CallProtocol> { self.protocol.clone() }
  #[doc(hidden)] pub fn _set_protocol(&mut self, protocol: CallProtocol) -> &mut Self { self.protocol = Some(protocol); self }
  
  pub fn connections(&self) -> Option<Vec<CallConnection>> { self.connections.clone() }
  #[doc(hidden)] pub fn _set_connections(&mut self, connections: Vec<CallConnection>) -> &mut Self { self.connections = Some(connections); self }
  
  pub fn config(&self) -> Option<String> { self.config.clone() }
  #[doc(hidden)] pub fn _set_config(&mut self, config: String) -> &mut Self { self.config = Some(config); self }
  
  pub fn encryption_key(&self) -> Option<String> { self.encryption_key.clone() }
  #[doc(hidden)] pub fn _set_encryption_key(&mut self, encryption_key: String) -> &mut Self { self.encryption_key = Some(encryption_key); self }
  
  pub fn emojis(&self) -> Option<Vec<String>> { self.emojis.clone() }
  #[doc(hidden)] pub fn _set_emojis(&mut self, emojis: Vec<String>) -> &mut Self { self.emojis = Some(emojis); self }
  
  pub fn allow_p2p(&self) -> Option<bool> { self.allow_p2p.clone() }
  #[doc(hidden)] pub fn _set_allow_p2p(&mut self, allow_p2p: bool) -> &mut Self { self.allow_p2p = Some(allow_p2p); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



