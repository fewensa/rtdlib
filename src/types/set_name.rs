
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the first and last name of the current user. If something changes, 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetName {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setName
  /// The new value of the first name for the user; 1-64 characters.
  first_name: Option<String>,
  /// The new value of the optional last name for the user; 0-64 characters.
  last_name: Option<String>,
  
}



impl Object for SetName {}
impl RObject for SetName {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setName" }
  fn td_type(&self) -> RTDType { RTDType::SetName }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetName {}


impl SetName {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setName".to_string(),
      first_name: None,
      last_name: None,
      
    }
  }
  
  pub fn first_name(&self) -> Option<String> { self.first_name.clone() }
  #[doc(hidden)] pub fn _set_first_name(&mut self, first_name: String) -> &mut Self { self.first_name = Some(first_name); self }
  
  pub fn last_name(&self) -> Option<String> { self.last_name.clone() }
  #[doc(hidden)] pub fn _set_last_name(&mut self, last_name: String) -> &mut Self { self.last_name = Some(last_name); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



