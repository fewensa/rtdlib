
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A chat member was deleted. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentChatDeleteMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentChatDeleteMember
  /// Name of the deleted member.
  member_name: Option<String>,
  /// True, if the current user was deleted from the group.
  is_current_user: Option<bool>,
  /// True, if the user has left the group himself.
  is_left: Option<bool>,
  
}



impl Object for PushMessageContentChatDeleteMember {}
impl RObject for PushMessageContentChatDeleteMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentChatDeleteMember" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentChatDeleteMember }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentChatDeleteMember {}


impl PushMessageContentChatDeleteMember {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentChatDeleteMember".to_string(),
      member_name: None,
      is_current_user: None,
      is_left: None,
      
    }
  }
  
  pub fn member_name(&self) -> Option<String> { self.member_name.clone() }
  #[doc(hidden)] pub fn _set_member_name(&mut self, member_name: String) -> &mut Self { self.member_name = Some(member_name); self }
  
  pub fn is_current_user(&self) -> Option<bool> { self.is_current_user.clone() }
  #[doc(hidden)] pub fn _set_is_current_user(&mut self, is_current_user: bool) -> &mut Self { self.is_current_user = Some(is_current_user); self }
  
  pub fn is_left(&self) -> Option<bool> { self.is_left.clone() }
  #[doc(hidden)] pub fn _set_is_left(&mut self, is_left: bool) -> &mut Self { self.is_left = Some(is_left); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



