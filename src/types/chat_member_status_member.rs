
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is a member of a chat, without any additional privileges or restrictions. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberStatusMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatMemberStatusMember
  
}



impl Object for ChatMemberStatusMember {}
impl RObject for ChatMemberStatusMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMemberStatusMember" }
  fn td_type(&self) -> RTDType { RTDType::ChatMemberStatusMember }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatMemberStatus for ChatMemberStatusMember {}


impl ChatMemberStatusMember {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatMemberStatusMember".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



