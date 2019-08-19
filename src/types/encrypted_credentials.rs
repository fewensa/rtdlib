
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains encrypted Telegram Passport data credentials. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedCredentials {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // encryptedCredentials
  /// The encrypted credentials.
  data: Option<String>,
  /// The decrypted data hash.
  hash: Option<String>,
  /// Secret for data decryption, encrypted with the service's public key.
  secret: Option<String>,
  
}



impl Object for EncryptedCredentials {}
impl RObject for EncryptedCredentials {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "encryptedCredentials" }
  fn td_type(&self) -> RTDType { RTDType::EncryptedCredentials }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl EncryptedCredentials {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "encryptedCredentials".to_string(),
      data: None,
      hash: None,
      secret: None,
      
    }
  }
  
  pub fn data(&self) -> Option<String> { self.data.clone() }
  #[doc(hidden)] pub fn _set_data(&mut self, data: String) -> &mut Self { self.data = Some(data); self }
  
  pub fn hash(&self) -> Option<String> { self.hash.clone() }
  #[doc(hidden)] pub fn _set_hash(&mut self, hash: String) -> &mut Self { self.hash = Some(hash); self }
  
  pub fn secret(&self) -> Option<String> { self.secret.clone() }
  #[doc(hidden)] pub fn _set_secret(&mut self, secret: String) -> &mut Self { self.secret = Some(secret); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



