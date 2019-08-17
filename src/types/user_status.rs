
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes the last time the user was online. 
#[typetag::serde(tag = "@struct")]
pub trait UserStatus: Object + RObject + Debug {}






impl UserStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<UserStatus> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDUserStatusType {
  UserStatusEmpty,
  UserStatusLastMonth,
  UserStatusLastWeek,
  UserStatusOffline,
  UserStatusOnline,
  UserStatusRecently,
  
}
impl RTDUserStatusType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDUserStatusType)(text.as_ref()) }
}



