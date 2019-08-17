
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is not a chat member. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberStatusLeft {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatMemberStatusLeft
  
}



impl Object for ChatMemberStatusLeft {}
impl RObject for ChatMemberStatusLeft {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMemberStatusLeft" }
  fn td_type(&self) -> RTDType { RTDType::ChatMemberStatusLeft }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatMemberStatus for ChatMemberStatusLeft {}


impl ChatMemberStatusLeft {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatMemberStatusLeft".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



