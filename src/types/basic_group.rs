
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users). 
#[derive(Debug, Serialize, Deserialize)]
pub struct BasicGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // basicGroup
  /// Group identifier.
  id: Option<i32>,
  /// Number of members in the group.
  member_count: Option<i32>,
  /// Status of the current user in the group.
  status: Option<Box<ChatMemberStatus>>,
  /// True, if all members have been granted administrator rights in the group.
  everyone_is_administrator: Option<bool>,
  /// True, if the group is active.
  is_active: Option<bool>,
  /// Identifier of the supergroup to which this group was upgraded; 0 if none.
  upgraded_to_supergroup_id: Option<i32>,
  
}


impl Clone for BasicGroup {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for BasicGroup {}
impl RObject for BasicGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "basicGroup" }
  fn td_type(&self) -> RTDType { RTDType::BasicGroup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl BasicGroup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "basicGroup".to_string(),
      id: None,
      member_count: None,
      status: None,
      everyone_is_administrator: None,
      is_active: None,
      upgraded_to_supergroup_id: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i32> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i32) -> &mut Self { self.id = Some(id); self }
  
  pub fn member_count(&self) -> Option<i32> { self.member_count.clone() }
  #[doc(hidden)] pub fn _set_member_count(&mut self, member_count: i32) -> &mut Self { self.member_count = Some(member_count); self }
  
  pub fn status(&self) -> Option<Box<ChatMemberStatus>> { self.status.clone() }
  #[doc(hidden)] pub fn _set_status(&mut self, status: Box<ChatMemberStatus>) -> &mut Self { self.status = Some(status); self }
  
  pub fn everyone_is_administrator(&self) -> Option<bool> { self.everyone_is_administrator.clone() }
  #[doc(hidden)] pub fn _set_everyone_is_administrator(&mut self, everyone_is_administrator: bool) -> &mut Self { self.everyone_is_administrator = Some(everyone_is_administrator); self }
  
  pub fn is_active(&self) -> Option<bool> { self.is_active.clone() }
  #[doc(hidden)] pub fn _set_is_active(&mut self, is_active: bool) -> &mut Self { self.is_active = Some(is_active); self }
  
  pub fn upgraded_to_supergroup_id(&self) -> Option<i32> { self.upgraded_to_supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_upgraded_to_supergroup_id(&mut self, upgraded_to_supergroup_id: i32) -> &mut Self { self.upgraded_to_supergroup_id = Some(upgraded_to_supergroup_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



