
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the database encryption key. Usually the encryption key is never changed and is stored in some OS keychain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDatabaseEncryptionKey {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setDatabaseEncryptionKey
  /// New encryption key.
  new_encryption_key: Option<String>,
  
}



impl Object for SetDatabaseEncryptionKey {}
impl RObject for SetDatabaseEncryptionKey {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setDatabaseEncryptionKey" }
  fn td_type(&self) -> RTDType { RTDType::SetDatabaseEncryptionKey }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetDatabaseEncryptionKey {}


impl SetDatabaseEncryptionKey {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setDatabaseEncryptionKey".to_string(),
      new_encryption_key: None,
      
    }
  }
  
  pub fn new_encryption_key(&self) -> Option<String> { self.new_encryption_key.clone() }
  #[doc(hidden)] pub fn _set_new_encryption_key(&mut self, new_encryption_key: String) -> &mut Self { self.new_encryption_key = Some(new_encryption_key); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



