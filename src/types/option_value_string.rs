
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a string option. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionValueString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // optionValueString
  /// The value of the option.
  value: Option<String>,
  
}



impl Object for OptionValueString {}
impl RObject for OptionValueString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "optionValueString" }
  fn td_type(&self) -> RTDType { RTDType::OptionValueString }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl OptionValue for OptionValueString {}


impl OptionValueString {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "optionValueString".to_string(),
      value: None,
      
    }
  }
  
  pub fn value(&self) -> Option<String> { self.value.clone() }
  #[doc(hidden)] pub fn _set_value(&mut self, value: String) -> &mut Self { self.value = Some(value); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



