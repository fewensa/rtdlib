
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes user privacy settings.
#[derive(Debug, Serialize, Deserialize)]
pub struct SetUserPrivacySettingRules {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setUserPrivacySettingRules
  /// The privacy setting.
  setting: Option<Box<UserPrivacySetting>>,
  /// The new privacy rules.
  rules: Option<UserPrivacySettingRules>,
  
}


impl Clone for SetUserPrivacySettingRules {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SetUserPrivacySettingRules {}
impl RObject for SetUserPrivacySettingRules {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setUserPrivacySettingRules" }
  fn td_type(&self) -> RTDType { RTDType::SetUserPrivacySettingRules }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetUserPrivacySettingRules {}


impl SetUserPrivacySettingRules {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setUserPrivacySettingRules".to_string(),
      setting: None,
      rules: None,
      
    }
  }
  
  pub fn setting(&self) -> Option<Box<UserPrivacySetting>> { self.setting.clone() }
  #[doc(hidden)] pub fn _set_setting(&mut self, setting: Box<UserPrivacySetting>) -> &mut Self { self.setting = Some(setting); self }
  
  pub fn rules(&self) -> Option<UserPrivacySettingRules> { self.rules.clone() }
  #[doc(hidden)] pub fn _set_rules(&mut self, rules: UserPrivacySettingRules) -> &mut Self { self.rules = Some(rules); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



