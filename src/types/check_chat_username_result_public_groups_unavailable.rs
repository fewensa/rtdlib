
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user can't be a member of a public supergroup. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckChatUsernameResultPublicGroupsUnavailable {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkChatUsernameResultPublicGroupsUnavailable
  
}



impl Object for CheckChatUsernameResultPublicGroupsUnavailable {}
impl RObject for CheckChatUsernameResultPublicGroupsUnavailable {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChatUsernameResultPublicGroupsUnavailable" }
  fn td_type(&self) -> RTDType { RTDType::CheckChatUsernameResultPublicGroupsUnavailable }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CheckChatUsernameResult for CheckChatUsernameResultPublicGroupsUnavailable {}


impl CheckChatUsernameResultPublicGroupsUnavailable {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkChatUsernameResultPublicGroupsUnavailable".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



