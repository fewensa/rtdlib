
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// TDLib needs an encryption key to decrypt the local database. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateWaitEncryptionKey {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // authorizationStateWaitEncryptionKey
  /// True, if the database is currently encrypted.
  is_encrypted: Option<bool>,
  
}



impl Object for AuthorizationStateWaitEncryptionKey {}
impl RObject for AuthorizationStateWaitEncryptionKey {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitEncryptionKey" }
  fn td_type(&self) -> RTDType { RTDType::AuthorizationStateWaitEncryptionKey }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl AuthorizationState for AuthorizationStateWaitEncryptionKey {}


impl AuthorizationStateWaitEncryptionKey {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "authorizationStateWaitEncryptionKey".to_string(),
      is_encrypted: None,
      
    }
  }
  
  pub fn is_encrypted(&self) -> Option<bool> { self.is_encrypted.clone() }
  #[doc(hidden)] pub fn _set_is_encrypted(&mut self, is_encrypted: bool) -> &mut Self { self.is_encrypted = Some(is_encrypted); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



