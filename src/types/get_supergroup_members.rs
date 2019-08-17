
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about members or banned users in a supergroup or channel. Can be used only if SupergroupFullInfo.can_get_members == true; additionally, administrator privileges may be required for some filters.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSupergroupMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getSupergroupMembers
  /// Identifier of the supergroup or channel.
  supergroup_id: Option<i32>,
  /// The type of users to return. By default, supergroupMembersRecent.
  filter: Option<Box<SupergroupMembersFilter>>,
  /// Number of users to skip.
  offset: Option<i32>,
  /// The maximum number of users be returned; up to 200.
  limit: Option<i32>,
  
}


impl Clone for GetSupergroupMembers {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for GetSupergroupMembers {}
impl RObject for GetSupergroupMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getSupergroupMembers" }
  fn td_type(&self) -> RTDType { RTDType::GetSupergroupMembers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetSupergroupMembers {}


impl GetSupergroupMembers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getSupergroupMembers".to_string(),
      supergroup_id: None,
      filter: None,
      offset: None,
      limit: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn filter(&self) -> Option<Box<SupergroupMembersFilter>> { self.filter.clone() }
  #[doc(hidden)] pub fn _set_filter(&mut self, filter: Box<SupergroupMembersFilter>) -> &mut Self { self.filter = Some(filter); self }
  
  pub fn offset(&self) -> Option<i32> { self.offset.clone() }
  #[doc(hidden)] pub fn _set_offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



