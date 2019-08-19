
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a secret chat. 
#[derive(Debug, Serialize, Deserialize)]
pub struct SecretChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // secretChat
  /// Secret chat identifier.
  id: Option<i32>,
  /// Identifier of the chat partner.
  user_id: Option<i32>,
  /// State of the secret chat.
  state: Option<Box<SecretChatState>>,
  /// True, if the chat was created by the current user; otherwise false.
  is_outbound: Option<bool>,
  /// Current message Time To Live setting (self-destruct timer) for the chat, in seconds.
  ttl: Option<i32>,
  /// Hash of the currently used key for comparison with the hash of the chat partner's key. This is a string of 36 bytes, which must be used to make a 12x12 square image with a color depth of 4. The first 16 bytes should be used to make a central 8x8 square, while the remaining 20 bytes should be used to construct a 2-pixel-wide border around that square. Alternatively, the first 32 bytes of the hash can be converted to the hexadecimal format and printed as 32 2-digit hex numbers.
  key_hash: Option<String>,
  /// Secret chat layer; determines features supported by the other client. Video notes are supported if the layer >= 66.
  layer: Option<i32>,
  
}


impl Clone for SecretChat {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SecretChat {}
impl RObject for SecretChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "secretChat" }
  fn td_type(&self) -> RTDType { RTDType::SecretChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl SecretChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "secretChat".to_string(),
      id: None,
      user_id: None,
      state: None,
      is_outbound: None,
      ttl: None,
      key_hash: None,
      layer: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i32> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i32) -> &mut Self { self.id = Some(id); self }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn state(&self) -> Option<Box<SecretChatState>> { self.state.clone() }
  #[doc(hidden)] pub fn _set_state(&mut self, state: Box<SecretChatState>) -> &mut Self { self.state = Some(state); self }
  
  pub fn is_outbound(&self) -> Option<bool> { self.is_outbound.clone() }
  #[doc(hidden)] pub fn _set_is_outbound(&mut self, is_outbound: bool) -> &mut Self { self.is_outbound = Some(is_outbound); self }
  
  pub fn ttl(&self) -> Option<i32> { self.ttl.clone() }
  #[doc(hidden)] pub fn _set_ttl(&mut self, ttl: i32) -> &mut Self { self.ttl = Some(ttl); self }
  
  pub fn key_hash(&self) -> Option<String> { self.key_hash.clone() }
  #[doc(hidden)] pub fn _set_key_hash(&mut self, key_hash: String) -> &mut Self { self.key_hash = Some(key_hash); self }
  
  pub fn layer(&self) -> Option<i32> { self.layer.clone() }
  #[doc(hidden)] pub fn _set_layer(&mut self, layer: i32) -> &mut Self { self.layer = Some(layer); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



