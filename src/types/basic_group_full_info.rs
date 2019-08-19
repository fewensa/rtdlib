
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains full information about a basic group. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicGroupFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // basicGroupFullInfo
  /// User identifier of the creator of the group; 0 if unknown.
  creator_user_id: Option<i32>,
  /// Group members.
  members: Option<Vec<ChatMember>>,
  /// Invite link for this group; available only for the group creator and only after it has been generated at least once.
  invite_link: Option<String>,
  
}



impl Object for BasicGroupFullInfo {}
impl RObject for BasicGroupFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "basicGroupFullInfo" }
  fn td_type(&self) -> RTDType { RTDType::BasicGroupFullInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl BasicGroupFullInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "basicGroupFullInfo".to_string(),
      creator_user_id: None,
      members: None,
      invite_link: None,
      
    }
  }
  
  pub fn creator_user_id(&self) -> Option<i32> { self.creator_user_id.clone() }
  #[doc(hidden)] pub fn _set_creator_user_id(&mut self, creator_user_id: i32) -> &mut Self { self.creator_user_id = Some(creator_user_id); self }
  
  pub fn members(&self) -> Option<Vec<ChatMember>> { self.members.clone() }
  #[doc(hidden)] pub fn _set_members(&mut self, members: Vec<ChatMember>) -> &mut Self { self.members = Some(members); self }
  
  pub fn invite_link(&self) -> Option<String> { self.invite_link.clone() }
  #[doc(hidden)] pub fn _set_invite_link(&mut self, invite_link: String) -> &mut Self { self.invite_link = Some(invite_link); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



