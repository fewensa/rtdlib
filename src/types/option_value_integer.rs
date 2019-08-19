
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents an integer option. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionValueInteger {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // optionValueInteger
  /// The value of the option.
  value: Option<i32>,
  
}



impl Object for OptionValueInteger {}
impl RObject for OptionValueInteger {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "optionValueInteger" }
  fn td_type(&self) -> RTDType { RTDType::OptionValueInteger }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl OptionValue for OptionValueInteger {}


impl OptionValueInteger {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "optionValueInteger".to_string(),
      value: None,
      
    }
  }
  
  pub fn value(&self) -> Option<i32> { self.value.clone() }
  #[doc(hidden)] pub fn _set_value(&mut self, value: i32) -> &mut Self { self.value = Some(value); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



