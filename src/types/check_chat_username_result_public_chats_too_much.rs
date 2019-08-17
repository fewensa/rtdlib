
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user has too much public chats, one of them should be made private first. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckChatUsernameResultPublicChatsTooMuch {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkChatUsernameResultPublicChatsTooMuch
  
}



impl Object for CheckChatUsernameResultPublicChatsTooMuch {}
impl RObject for CheckChatUsernameResultPublicChatsTooMuch {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChatUsernameResultPublicChatsTooMuch" }
  fn td_type(&self) -> RTDType { RTDType::CheckChatUsernameResultPublicChatsTooMuch }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CheckChatUsernameResult for CheckChatUsernameResultPublicChatsTooMuch {}


impl CheckChatUsernameResultPublicChatsTooMuch {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkChatUsernameResultPublicChatsTooMuch".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



