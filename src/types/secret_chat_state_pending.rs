
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The secret chat is not yet created; waiting for the other user to get online. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretChatStatePending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // secretChatStatePending
  
}



impl Object for SecretChatStatePending {}
impl RObject for SecretChatStatePending {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "secretChatStatePending" }
  fn td_type(&self) -> RTDType { RTDType::SecretChatStatePending }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SecretChatState for SecretChatStatePending {}


impl SecretChatStatePending {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "secretChatStatePending".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



