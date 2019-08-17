
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Closes a secret chat, effectively transfering its state to 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseSecretChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // closeSecretChat
  /// Secret chat identifier.
  secret_chat_id: Option<i32>,
  
}



impl Object for CloseSecretChat {}
impl RObject for CloseSecretChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "closeSecretChat" }
  fn td_type(&self) -> RTDType { RTDType::CloseSecretChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CloseSecretChat {}


impl CloseSecretChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "closeSecretChat".to_string(),
      secret_chat_id: None,
      
    }
  }
  
  pub fn secret_chat_id(&self) -> Option<i32> { self.secret_chat_id.clone() }
  #[doc(hidden)] pub fn _set_secret_chat_id(&mut self, secret_chat_id: i32) -> &mut Self { self.secret_chat_id = Some(secret_chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



