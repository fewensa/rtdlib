
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents an unknown option or an option which has a default value. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionValueEmpty {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // optionValueEmpty
  
}



impl Object for OptionValueEmpty {}
impl RObject for OptionValueEmpty {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "optionValueEmpty" }
  fn td_type(&self) -> RTDType { RTDType::OptionValueEmpty }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl OptionValue for OptionValueEmpty {}


impl OptionValueEmpty {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "optionValueEmpty".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



