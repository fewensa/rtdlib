
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the value of an option by its name. (Check the list of available options on 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getOption
  /// The name of the option.
  name: Option<String>,
  
}



impl Object for GetOption {}
impl RObject for GetOption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getOption" }
  fn td_type(&self) -> RTDType { RTDType::GetOption }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetOption {}


impl GetOption {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getOption".to_string(),
      name: None,
      
    }
  }
  
  pub fn name(&self) -> Option<String> { self.name.clone() }
  #[doc(hidden)] pub fn _set_name(&mut self, name: String) -> &mut Self { self.name = Some(name); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



