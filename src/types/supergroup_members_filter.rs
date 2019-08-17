
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Specifies the kind of chat members to return in 
#[typetag::serde(tag = "@struct")]
pub trait SupergroupMembersFilter: Object + RObject + Debug {}






impl SupergroupMembersFilter {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<SupergroupMembersFilter> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDSupergroupMembersFilterType {
  SupergroupMembersFilterAdministrators,
  SupergroupMembersFilterBanned,
  SupergroupMembersFilterBots,
  SupergroupMembersFilterRecent,
  SupergroupMembersFilterRestricted,
  SupergroupMembersFilterSearch,
  
}
impl RTDSupergroupMembersFilterType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDSupergroupMembersFilterType)(text.as_ref()) }
}



