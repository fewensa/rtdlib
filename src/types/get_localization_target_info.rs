
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about the current localization target. This is an offline request if only_local is true. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLocalizationTargetInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getLocalizationTargetInfo
  /// If true, returns only locally available information without sending network requests.
  only_local: Option<bool>,
  
}



impl Object for GetLocalizationTargetInfo {}
impl RObject for GetLocalizationTargetInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLocalizationTargetInfo" }
  fn td_type(&self) -> RTDType { RTDType::GetLocalizationTargetInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetLocalizationTargetInfo {}


impl GetLocalizationTargetInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getLocalizationTargetInfo".to_string(),
      only_local: None,
      
    }
  }
  
  pub fn only_local(&self) -> Option<bool> { self.only_local.clone() }
  #[doc(hidden)] pub fn _set_only_local(&mut self, only_local: bool) -> &mut Self { self.only_local = Some(only_local); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



