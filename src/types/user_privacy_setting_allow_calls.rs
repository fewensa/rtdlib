
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A privacy setting for managing whether the user can be called. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowCalls {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userPrivacySettingAllowCalls
  
}



impl Object for UserPrivacySettingAllowCalls {}
impl RObject for UserPrivacySettingAllowCalls {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingAllowCalls" }
  fn td_type(&self) -> RTDType { RTDType::UserPrivacySettingAllowCalls }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserPrivacySetting for UserPrivacySettingAllowCalls {}


impl UserPrivacySettingAllowCalls {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userPrivacySettingAllowCalls".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



