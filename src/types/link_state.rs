
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents the relationship between user A and user B. For incoming_link, user A is the current user; for outgoing_link, user B is the current user. 
#[typetag::serde(tag = "@struct")]
pub trait LinkState: Object + RObject + Debug {}






impl LinkState {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<LinkState> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDLinkStateType {
  LinkStateIsContact,
  LinkStateKnowsPhoneNumber,
  LinkStateNone,
  
}
impl RTDLinkStateType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDLinkStateType)(text.as_ref()) }
}



