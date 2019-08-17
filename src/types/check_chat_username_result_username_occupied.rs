
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The username is occupied. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckChatUsernameResultUsernameOccupied {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkChatUsernameResultUsernameOccupied
  
}



impl Object for CheckChatUsernameResultUsernameOccupied {}
impl RObject for CheckChatUsernameResultUsernameOccupied {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChatUsernameResultUsernameOccupied" }
  fn td_type(&self) -> RTDType { RTDType::CheckChatUsernameResultUsernameOccupied }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CheckChatUsernameResult for CheckChatUsernameResultUsernameOccupied {}


impl CheckChatUsernameResultUsernameOccupied {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkChatUsernameResultUsernameOccupied".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



