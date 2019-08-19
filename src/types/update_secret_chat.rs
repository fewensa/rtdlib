
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the client. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSecretChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateSecretChat
  /// New data about the secret chat.
  secret_chat: Option<SecretChat>,
  
}



impl Object for UpdateSecretChat {}
impl RObject for UpdateSecretChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateSecretChat" }
  fn td_type(&self) -> RTDType { RTDType::UpdateSecretChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateSecretChat {}


impl UpdateSecretChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateSecretChat".to_string(),
      secret_chat: None,
      
    }
  }
  
  pub fn secret_chat(&self) -> Option<SecretChat> { self.secret_chat.clone() }
  #[doc(hidden)] pub fn _set_secret_chat(&mut self, secret_chat: SecretChat) -> &mut Self { self.secret_chat = Some(secret_chat); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



