
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// New chat members were invited to a group. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentChatAddMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentChatAddMembers
  /// Name of the added member.
  member_name: Option<String>,
  /// True, if the current user was added to the group.
  is_current_user: Option<bool>,
  /// True, if the user has returned to the group himself.
  is_returned: Option<bool>,
  
}



impl Object for PushMessageContentChatAddMembers {}
impl RObject for PushMessageContentChatAddMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentChatAddMembers" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentChatAddMembers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentChatAddMembers {}


impl PushMessageContentChatAddMembers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentChatAddMembers".to_string(),
      member_name: None,
      is_current_user: None,
      is_returned: None,
      
    }
  }
  
  pub fn member_name(&self) -> Option<String> { self.member_name.clone() }
  #[doc(hidden)] pub fn _set_member_name(&mut self, member_name: String) -> &mut Self { self.member_name = Some(member_name); self }
  
  pub fn is_current_user(&self) -> Option<bool> { self.is_current_user.clone() }
  #[doc(hidden)] pub fn _set_is_current_user(&mut self, is_current_user: bool) -> &mut Self { self.is_current_user = Some(is_current_user); self }
  
  pub fn is_returned(&self) -> Option<bool> { self.is_returned.clone() }
  #[doc(hidden)] pub fn _set_is_returned(&mut self, is_returned: bool) -> &mut Self { self.is_returned = Some(is_returned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



