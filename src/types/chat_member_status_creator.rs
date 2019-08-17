
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is the creator of a chat and has all the administrator privileges. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberStatusCreator {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatMemberStatusCreator
  /// True, if the user is a member of the chat.
  is_member: Option<bool>,
  
}



impl Object for ChatMemberStatusCreator {}
impl RObject for ChatMemberStatusCreator {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMemberStatusCreator" }
  fn td_type(&self) -> RTDType { RTDType::ChatMemberStatusCreator }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatMemberStatus for ChatMemberStatusCreator {}


impl ChatMemberStatusCreator {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatMemberStatusCreator".to_string(),
      is_member: None,
      
    }
  }
  
  pub fn is_member(&self) -> Option<bool> { self.is_member.clone() }
  #[doc(hidden)] pub fn _set_is_member(&mut self, is_member: bool) -> &mut Self { self.is_member = Some(is_member); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



