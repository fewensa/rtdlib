
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Checks the database encryption key for correctness. Works only when the current authorization state is 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckDatabaseEncryptionKey {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkDatabaseEncryptionKey
  /// Encryption key to check or set up.
  encryption_key: Option<String>,
  
}



impl Object for CheckDatabaseEncryptionKey {}
impl RObject for CheckDatabaseEncryptionKey {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkDatabaseEncryptionKey" }
  fn td_type(&self) -> RTDType { RTDType::CheckDatabaseEncryptionKey }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CheckDatabaseEncryptionKey {}


impl CheckDatabaseEncryptionKey {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkDatabaseEncryptionKey".to_string(),
      encryption_key: None,
      
    }
  }
  
  pub fn encryption_key(&self) -> Option<String> { self.encryption_key.clone() }
  #[doc(hidden)] pub fn _set_encryption_key(&mut self, encryption_key: String) -> &mut Self { self.encryption_key = Some(encryption_key); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



