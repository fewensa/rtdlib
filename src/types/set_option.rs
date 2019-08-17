
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sets the value of an option. (Check the list of available options on 
#[derive(Debug, Serialize, Deserialize)]
pub struct SetOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setOption
  /// The name of the option.
  name: Option<String>,
  /// The new value of the option.
  value: Option<Box<OptionValue>>,
  
}


impl Clone for SetOption {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SetOption {}
impl RObject for SetOption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setOption" }
  fn td_type(&self) -> RTDType { RTDType::SetOption }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetOption {}


impl SetOption {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setOption".to_string(),
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



