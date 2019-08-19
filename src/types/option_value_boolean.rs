
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a boolean option. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionValueBoolean {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // optionValueBoolean
  /// The value of the option.
  value: Option<bool>,
  
}



impl Object for OptionValueBoolean {}
impl RObject for OptionValueBoolean {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "optionValueBoolean" }
  fn td_type(&self) -> RTDType { RTDType::OptionValueBoolean }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl OptionValue for OptionValueBoolean {}


impl OptionValueBoolean {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "optionValueBoolean".to_string(),
      value: None,
      
    }
  }
  
  pub fn value(&self) -> Option<bool> { self.value.clone() }
  #[doc(hidden)] pub fn _set_value(&mut self, value: bool) -> &mut Self { self.value = Some(value); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



