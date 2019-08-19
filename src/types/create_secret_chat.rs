
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns an existing chat corresponding to a known secret chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSecretChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // createSecretChat
  /// Secret chat identifier.
  secret_chat_id: Option<i32>,
  
}



impl Object for CreateSecretChat {}
impl RObject for CreateSecretChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createSecretChat" }
  fn td_type(&self) -> RTDType { RTDType::CreateSecretChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CreateSecretChat {}


impl CreateSecretChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "createSecretChat".to_string(),
      secret_chat_id: None,
      
    }
  }
  
  pub fn secret_chat_id(&self) -> Option<i32> { self.secret_chat_id.clone() }
  #[doc(hidden)] pub fn _set_secret_chat_id(&mut self, secret_chat_id: i32) -> &mut Self { self.secret_chat_id = Some(secret_chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



