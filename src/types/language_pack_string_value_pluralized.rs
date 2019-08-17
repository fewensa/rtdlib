
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A language pack string which has different forms based on the number of some object it mentions. See 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePackStringValuePluralized {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // languagePackStringValuePluralized
  /// Value for zero objects.
  zero_value: Option<String>,
  /// Value for one object.
  one_value: Option<String>,
  /// Value for two objects.
  two_value: Option<String>,
  /// Value for few objects.
  few_value: Option<String>,
  /// Value for many objects.
  many_value: Option<String>,
  /// Default value.
  other_value: Option<String>,
  
}



impl Object for LanguagePackStringValuePluralized {}
impl RObject for LanguagePackStringValuePluralized {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "languagePackStringValuePluralized" }
  fn td_type(&self) -> RTDType { RTDType::LanguagePackStringValuePluralized }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl LanguagePackStringValue for LanguagePackStringValuePluralized {}


impl LanguagePackStringValuePluralized {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "languagePackStringValuePluralized".to_string(),
      zero_value: None,
      one_value: None,
      two_value: None,
      few_value: None,
      many_value: None,
      other_value: None,
      
    }
  }
  
  pub fn zero_value(&self) -> Option<String> { self.zero_value.clone() }
  #[doc(hidden)] pub fn _set_zero_value(&mut self, zero_value: String) -> &mut Self { self.zero_value = Some(zero_value); self }
  
  pub fn one_value(&self) -> Option<String> { self.one_value.clone() }
  #[doc(hidden)] pub fn _set_one_value(&mut self, one_value: String) -> &mut Self { self.one_value = Some(one_value); self }
  
  pub fn two_value(&self) -> Option<String> { self.two_value.clone() }
  #[doc(hidden)] pub fn _set_two_value(&mut self, two_value: String) -> &mut Self { self.two_value = Some(two_value); self }
  
  pub fn few_value(&self) -> Option<String> { self.few_value.clone() }
  #[doc(hidden)] pub fn _set_few_value(&mut self, few_value: String) -> &mut Self { self.few_value = Some(few_value); self }
  
  pub fn many_value(&self) -> Option<String> { self.many_value.clone() }
  #[doc(hidden)] pub fn _set_many_value(&mut self, many_value: String) -> &mut Self { self.many_value = Some(many_value); self }
  
  pub fn other_value(&self) -> Option<String> { self.other_value.clone() }
  #[doc(hidden)] pub fn _set_other_value(&mut self, other_value: String) -> &mut Self { self.other_value = Some(other_value); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



