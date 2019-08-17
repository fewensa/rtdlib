
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A rule to allow all of a user's contacts to do something. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userPrivacySettingRuleAllowContacts
  
}



impl Object for UserPrivacySettingRuleAllowContacts {}
impl RObject for UserPrivacySettingRuleAllowContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRuleAllowContacts" }
  fn td_type(&self) -> RTDType { RTDType::UserPrivacySettingRuleAllowContacts }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserPrivacySettingRule for UserPrivacySettingRuleAllowContacts {}


impl UserPrivacySettingRuleAllowContacts {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userPrivacySettingRuleAllowContacts".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



