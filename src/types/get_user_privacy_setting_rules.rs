
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the current privacy settings.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserPrivacySettingRules {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getUserPrivacySettingRules
  /// The privacy setting.
  setting: Option<Box<UserPrivacySetting>>,
  
}


impl Clone for GetUserPrivacySettingRules {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for GetUserPrivacySettingRules {}
impl RObject for GetUserPrivacySettingRules {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getUserPrivacySettingRules" }
  fn td_type(&self) -> RTDType { RTDType::GetUserPrivacySettingRules }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetUserPrivacySettingRules {}


impl GetUserPrivacySettingRules {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getUserPrivacySettingRules".to_string(),
      setting: None,
      
    }
  }
  
  pub fn setting(&self) -> Option<Box<UserPrivacySetting>> { self.setting.clone() }
  #[doc(hidden)] pub fn _set_setting(&mut self, setting: Box<UserPrivacySetting>) -> &mut Self { self.setting = Some(setting); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



