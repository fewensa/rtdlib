
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed. 
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPrivacySettingRules {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userPrivacySettingRules
  /// A list of rules.
  rules: Option<Vec<Box<UserPrivacySettingRule>>>,
  
}


impl Clone for UserPrivacySettingRules {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UserPrivacySettingRules {}
impl RObject for UserPrivacySettingRules {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRules" }
  fn td_type(&self) -> RTDType { RTDType::UserPrivacySettingRules }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl UserPrivacySettingRules {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userPrivacySettingRules".to_string(),
      rules: None,
      
    }
  }
  
  pub fn rules(&self) -> Option<Vec<Box<UserPrivacySettingRule>>> { self.rules.clone() }
  #[doc(hidden)] pub fn _set_rules(&mut self, rules: Vec<Box<UserPrivacySettingRule>>) -> &mut Self { self.rules = Some(rules); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



