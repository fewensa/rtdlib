
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A rule to allow all users to do something. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowAll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userPrivacySettingRuleAllowAll
  
}



impl Object for UserPrivacySettingRuleAllowAll {}
impl RObject for UserPrivacySettingRuleAllowAll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRuleAllowAll" }
  fn td_type(&self) -> RTDType { RTDType::UserPrivacySettingRuleAllowAll }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserPrivacySettingRule for UserPrivacySettingRuleAllowAll {}


impl UserPrivacySettingRuleAllowAll {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userPrivacySettingRuleAllowAll".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



