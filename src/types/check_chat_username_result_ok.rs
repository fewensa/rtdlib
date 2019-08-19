
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The username can be set. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckChatUsernameResultOk {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkChatUsernameResultOk
  
}



impl Object for CheckChatUsernameResultOk {}
impl RObject for CheckChatUsernameResultOk {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChatUsernameResultOk" }
  fn td_type(&self) -> RTDType { RTDType::CheckChatUsernameResultOk }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CheckChatUsernameResult for CheckChatUsernameResultOk {}


impl CheckChatUsernameResultOk {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkChatUsernameResultOk".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



