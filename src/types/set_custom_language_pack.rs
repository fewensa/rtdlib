
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds or changes a custom local language pack to the current localization target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCustomLanguagePack {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setCustomLanguagePack
  /// Information about the language pack. Language pack ID must start with 'X', consist only of English letters, digits and hyphens, and must not exceed 64 characters. Can be called before authorization.
  info: Option<LanguagePackInfo>,
  /// Strings of the new language pack.
  strings: Option<Vec<LanguagePackString>>,
  
}



impl Object for SetCustomLanguagePack {}
impl RObject for SetCustomLanguagePack {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setCustomLanguagePack" }
  fn td_type(&self) -> RTDType { RTDType::SetCustomLanguagePack }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetCustomLanguagePack {}


impl SetCustomLanguagePack {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setCustomLanguagePack".to_string(),
      info: None,
      strings: None,
      
    }
  }
  
  pub fn info(&self) -> Option<LanguagePackInfo> { self.info.clone() }
  #[doc(hidden)] pub fn _set_info(&mut self, info: LanguagePackInfo) -> &mut Self { self.info = Some(info); self }
  
  pub fn strings(&self) -> Option<Vec<LanguagePackString>> { self.strings.clone() }
  #[doc(hidden)] pub fn _set_strings(&mut self, strings: Vec<LanguagePackString>) -> &mut Self { self.strings = Some(strings); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



