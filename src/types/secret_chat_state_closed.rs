
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The secret chat is closed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretChatStateClosed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // secretChatStateClosed
  
}



impl Object for SecretChatStateClosed {}
impl RObject for SecretChatStateClosed {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "secretChatStateClosed" }
  fn td_type(&self) -> RTDType { RTDType::SecretChatStateClosed }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SecretChatState for SecretChatStateClosed {}


impl SecretChatStateClosed {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "secretChatStateClosed".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



