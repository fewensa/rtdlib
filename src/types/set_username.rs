
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the username of the current user. If something changes, 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetUsername {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setUsername
  /// The new value of the username. Use an empty string to remove the username.
  username: Option<String>,
  
}



impl Object for SetUsername {}
impl RObject for SetUsername {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setUsername" }
  fn td_type(&self) -> RTDType { RTDType::SetUsername }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetUsername {}


impl SetUsername {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setUsername".to_string(),
      username: None,
      
    }
  }
  
  pub fn username(&self) -> Option<String> { self.username.clone() }
  #[doc(hidden)] pub fn _set_username(&mut self, username: String) -> &mut Self { self.username = Some(username); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



