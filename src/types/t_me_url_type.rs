
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes the type of a URL linking to an internal Telegram entity. 
#[typetag::serde(tag = "@struct")]
pub trait TMeUrlType: Object + RObject + Debug {}






impl TMeUrlType {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<TMeUrlType> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDTMeUrlTypeType {
  TMeUrlTypeChatInvite,
  TMeUrlTypeStickerSet,
  TMeUrlTypeSupergroup,
  TMeUrlTypeUser,
  
}
impl RTDTMeUrlTypeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDTMeUrlTypeType)(text.as_ref()) }
}



