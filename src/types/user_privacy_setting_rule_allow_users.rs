
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A rule to allow certain specified users to do something. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowUsers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userPrivacySettingRuleAllowUsers
  /// The user identifiers.
  user_ids: Option<Vec<i32>>,
  
}



impl Object for UserPrivacySettingRuleAllowUsers {}
impl RObject for UserPrivacySettingRuleAllowUsers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRuleAllowUsers" }
  fn td_type(&self) -> RTDType { RTDType::UserPrivacySettingRuleAllowUsers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserPrivacySettingRule for UserPrivacySettingRuleAllowUsers {}


impl UserPrivacySettingRuleAllowUsers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userPrivacySettingRuleAllowUsers".to_string(),
      user_ids: None,
      
    }
  }
  
  pub fn user_ids(&self) -> Option<Vec<i32>> { self.user_ids.clone() }
  #[doc(hidden)] pub fn _set_user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self { self.user_ids = Some(user_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



