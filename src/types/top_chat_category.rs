
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents the categories of chats for which a list of frequently used chats can be retrieved. 
#[typetag::serde(tag = "@struct")]
pub trait TopChatCategory: Object + RObject + Debug {}






impl TopChatCategory {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<TopChatCategory> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDTopChatCategoryType {
  TopChatCategoryBots,
  TopChatCategoryCalls,
  TopChatCategoryChannels,
  TopChatCategoryGroups,
  TopChatCategoryInlineBots,
  TopChatCategoryUsers,
  
}
impl RTDTopChatCategoryType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDTopChatCategoryType)(text.as_ref()) }
}



