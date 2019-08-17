
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A privacy setting for managing whether the user can be invited to chats. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowChatInvites {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userPrivacySettingAllowChatInvites
  
}



impl Object for UserPrivacySettingAllowChatInvites {}
impl RObject for UserPrivacySettingAllowChatInvites {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingAllowChatInvites" }
  fn td_type(&self) -> RTDType { RTDType::UserPrivacySettingAllowChatInvites }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserPrivacySetting for UserPrivacySettingAllowChatInvites {}


impl UserPrivacySettingAllowChatInvites {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userPrivacySettingAllowChatInvites".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



