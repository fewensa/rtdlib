
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The secret chat is ready to use. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretChatStateReady {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // secretChatStateReady
  
}



impl Object for SecretChatStateReady {}
impl RObject for SecretChatStateReady {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "secretChatStateReady" }
  fn td_type(&self) -> RTDType { RTDType::SecretChatStateReady }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SecretChatState for SecretChatStateReady {}


impl SecretChatStateReady {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "secretChatStateReady".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



