
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes available user privacy settings. 
#[typetag::serde(tag = "@struct")]
pub trait UserPrivacySetting: Object + RObject + Debug {}






impl UserPrivacySetting {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<UserPrivacySetting> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDUserPrivacySettingType {
  UserPrivacySettingAllowCalls,
  UserPrivacySettingAllowChatInvites,
  UserPrivacySettingAllowPeerToPeerCalls,
  UserPrivacySettingShowStatus,
  
}
impl RTDUserPrivacySettingType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDUserPrivacySettingType)(text.as_ref()) }
}



