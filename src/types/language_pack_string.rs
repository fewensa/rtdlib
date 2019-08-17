
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents one language pack string. 
#[derive(Debug, Serialize, Deserialize)]
pub struct LanguagePackString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // languagePackString
  /// String key.
  key: Option<String>,
  /// String value.
  value: Option<Box<LanguagePackStringValue>>,
  
}


impl Clone for LanguagePackString {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for LanguagePackString {}
impl RObject for LanguagePackString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "languagePackString" }
  fn td_type(&self) -> RTDType { RTDType::LanguagePackString }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl LanguagePackString {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "languagePackString".to_string(),
      key: None,
      value: None,
      
    }
  }
  
  pub fn key(&self) -> Option<String> { self.key.clone() }
  #[doc(hidden)] pub fn _set_key(&mut self, key: String) -> &mut Self { self.key = Some(key); self }
  
  pub fn value(&self) -> Option<Box<LanguagePackStringValue>> { self.value.clone() }
  #[doc(hidden)] pub fn _set_value(&mut self, value: Box<LanguagePackStringValue>) -> &mut Self { self.value = Some(value); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



