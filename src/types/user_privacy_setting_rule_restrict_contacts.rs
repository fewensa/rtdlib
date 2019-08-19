
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A rule to restrict all contacts of a user from doing something. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userPrivacySettingRuleRestrictContacts
  
}



impl Object for UserPrivacySettingRuleRestrictContacts {}
impl RObject for UserPrivacySettingRuleRestrictContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRuleRestrictContacts" }
  fn td_type(&self) -> RTDType { RTDType::UserPrivacySettingRuleRestrictContacts }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserPrivacySettingRule for UserPrivacySettingRuleRestrictContacts {}


impl UserPrivacySettingRuleRestrictContacts {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userPrivacySettingRuleRestrictContacts".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



