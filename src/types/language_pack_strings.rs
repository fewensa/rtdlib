
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a list of language pack strings. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePackStrings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // languagePackStrings
  /// A list of language pack strings.
  strings: Option<Vec<LanguagePackString>>,
  
}



impl Object for LanguagePackStrings {}
impl RObject for LanguagePackStrings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "languagePackStrings" }
  fn td_type(&self) -> RTDType { RTDType::LanguagePackStrings }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl LanguagePackStrings {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "languagePackStrings".to_string(),
      strings: None,
      
    }
  }
  
  pub fn strings(&self) -> Option<Vec<LanguagePackString>> { self.strings.clone() }
  #[doc(hidden)] pub fn _set_strings(&mut self, strings: Vec<LanguagePackString>) -> &mut Self { self.strings = Some(strings); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



