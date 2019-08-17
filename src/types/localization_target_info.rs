
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about the current localization target. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizationTargetInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // localizationTargetInfo
  /// List of available language packs for this application.
  language_packs: Option<Vec<LanguagePackInfo>>,
  
}



impl Object for LocalizationTargetInfo {}
impl RObject for LocalizationTargetInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "localizationTargetInfo" }
  fn td_type(&self) -> RTDType { RTDType::LocalizationTargetInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl LocalizationTargetInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "localizationTargetInfo".to_string(),
      language_packs: None,
      
    }
  }
  
  pub fn language_packs(&self) -> Option<Vec<LanguagePackInfo>> { self.language_packs.clone() }
  #[doc(hidden)] pub fn _set_language_packs(&mut self, language_packs: Vec<LanguagePackInfo>) -> &mut Self { self.language_packs = Some(language_packs); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



