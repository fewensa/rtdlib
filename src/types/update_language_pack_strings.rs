
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Some language pack strings have been updated. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLanguagePackStrings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateLanguagePackStrings
  /// Localization target to which the language pack belongs.
  localization_target: Option<String>,
  /// Identifier of the updated language pack.
  language_pack_id: Option<String>,
  /// List of changed language pack strings.
  strings: Option<Vec<LanguagePackString>>,
  
}



impl Object for UpdateLanguagePackStrings {}
impl RObject for UpdateLanguagePackStrings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateLanguagePackStrings" }
  fn td_type(&self) -> RTDType { RTDType::UpdateLanguagePackStrings }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateLanguagePackStrings {}


impl UpdateLanguagePackStrings {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateLanguagePackStrings".to_string(),
      localization_target: None,
      language_pack_id: None,
      strings: None,
      
    }
  }
  
  pub fn localization_target(&self) -> Option<String> { self.localization_target.clone() }
  #[doc(hidden)] pub fn _set_localization_target(&mut self, localization_target: String) -> &mut Self { self.localization_target = Some(localization_target); self }
  
  pub fn language_pack_id(&self) -> Option<String> { self.language_pack_id.clone() }
  #[doc(hidden)] pub fn _set_language_pack_id(&mut self, language_pack_id: String) -> &mut Self { self.language_pack_id = Some(language_pack_id); self }
  
  pub fn strings(&self) -> Option<Vec<LanguagePackString>> { self.strings.clone() }
  #[doc(hidden)] pub fn _set_strings(&mut self, strings: Vec<LanguagePackString>) -> &mut Self { self.strings = Some(strings); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



