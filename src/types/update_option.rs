
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An option changed its value. 
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateOption
  /// The option name.
  name: Option<String>,
  /// The new option value.
  value: Option<Box<OptionValue>>,
  
}


impl Clone for UpdateOption {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UpdateOption {}
impl RObject for UpdateOption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateOption" }
  fn td_type(&self) -> RTDType { RTDType::UpdateOption }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateOption {}


impl UpdateOption {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateOption".to_string(),
      name: None,
      value: None,
      
    }
  }
  
  pub fn name(&self) -> Option<String> { self.name.clone() }
  #[doc(hidden)] pub fn _set_name(&mut self, name: String) -> &mut Self { self.name = Some(name); self }
  
  pub fn value(&self) -> Option<Box<OptionValue>> { self.value.clone() }
  #[doc(hidden)] pub fn _set_value(&mut self, value: Box<OptionValue>) -> &mut Self { self.value = Some(value); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



