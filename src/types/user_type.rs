
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents the type of the user. The following types are possible: regular users, deleted users and bots. 
#[typetag::serde(tag = "@struct")]
pub trait UserType: Object + RObject + Debug {}






impl UserType {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<UserType> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDUserTypeType {
  UserTypeBot,
  UserTypeDeleted,
  UserTypeRegular,
  UserTypeUnknown,
  
}
impl RTDUserTypeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDUserTypeType)(text.as_ref()) }
}



