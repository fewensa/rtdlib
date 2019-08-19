
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A rule to restrict all users from doing something. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictAll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userPrivacySettingRuleRestrictAll
  
}



impl Object for UserPrivacySettingRuleRestrictAll {}
impl RObject for UserPrivacySettingRuleRestrictAll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRuleRestrictAll" }
  fn td_type(&self) -> RTDType { RTDType::UserPrivacySettingRuleRestrictAll }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserPrivacySettingRule for UserPrivacySettingRuleRestrictAll {}


impl UserPrivacySettingRuleRestrictAll {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userPrivacySettingRuleRestrictAll".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



