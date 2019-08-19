
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A privacy setting for managing whether the user's online status is visible. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivacySettingShowStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userPrivacySettingShowStatus
  
}



impl Object for UserPrivacySettingShowStatus {}
impl RObject for UserPrivacySettingShowStatus {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingShowStatus" }
  fn td_type(&self) -> RTDType { RTDType::UserPrivacySettingShowStatus }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserPrivacySetting for UserPrivacySettingShowStatus {}


impl UserPrivacySettingShowStatus {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userPrivacySettingShowStatus".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



