
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A privacy setting for managing whether peer-to-peer connections can be used for calls. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowPeerToPeerCalls {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userPrivacySettingAllowPeerToPeerCalls
  
}



impl Object for UserPrivacySettingAllowPeerToPeerCalls {}
impl RObject for UserPrivacySettingAllowPeerToPeerCalls {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingAllowPeerToPeerCalls" }
  fn td_type(&self) -> RTDType { RTDType::UserPrivacySettingAllowPeerToPeerCalls }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserPrivacySetting for UserPrivacySettingAllowPeerToPeerCalls {}


impl UserPrivacySettingAllowPeerToPeerCalls {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userPrivacySettingAllowPeerToPeerCalls".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



