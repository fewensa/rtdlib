
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents a single rule for managing privacy settings. 
#[typetag::serde(tag = "@struct")]
pub trait UserPrivacySettingRule: Object + RObject + Debug {}






impl UserPrivacySettingRule {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<UserPrivacySettingRule> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDUserPrivacySettingRuleType {
  UserPrivacySettingRuleAllowAll,
  UserPrivacySettingRuleAllowContacts,
  UserPrivacySettingRuleAllowUsers,
  UserPrivacySettingRuleRestrictAll,
  UserPrivacySettingRuleRestrictContacts,
  UserPrivacySettingRuleRestrictUsers,
  
}
impl RTDUserPrivacySettingRuleType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDUserPrivacySettingRuleType)(text.as_ref()) }
}



