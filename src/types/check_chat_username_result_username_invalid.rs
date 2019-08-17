
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The username is invalid. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckChatUsernameResultUsernameInvalid {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkChatUsernameResultUsernameInvalid
  
}



impl Object for CheckChatUsernameResultUsernameInvalid {}
impl RObject for CheckChatUsernameResultUsernameInvalid {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChatUsernameResultUsernameInvalid" }
  fn td_type(&self) -> RTDType { RTDType::CheckChatUsernameResultUsernameInvalid }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CheckChatUsernameResult for CheckChatUsernameResultUsernameInvalid {}


impl CheckChatUsernameResultUsernameInvalid {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkChatUsernameResultUsernameInvalid".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



