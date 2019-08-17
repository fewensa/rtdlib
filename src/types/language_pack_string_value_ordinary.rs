
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An ordinary language pack string. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePackStringValueOrdinary {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // languagePackStringValueOrdinary
  /// String value.
  value: Option<String>,
  
}



impl Object for LanguagePackStringValueOrdinary {}
impl RObject for LanguagePackStringValueOrdinary {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "languagePackStringValueOrdinary" }
  fn td_type(&self) -> RTDType { RTDType::LanguagePackStringValueOrdinary }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl LanguagePackStringValue for LanguagePackStringValueOrdinary {}


impl LanguagePackStringValueOrdinary {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "languagePackStringValueOrdinary".to_string(),
      value: None,
      
    }
  }
  
  pub fn value(&self) -> Option<String> { self.value.clone() }
  #[doc(hidden)] pub fn _set_value(&mut self, value: String) -> &mut Self { self.value = Some(value); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



