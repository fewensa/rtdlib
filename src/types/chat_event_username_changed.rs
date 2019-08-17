
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The chat username was changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventUsernameChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventUsernameChanged
  /// Previous chat username.
  old_username: Option<String>,
  /// New chat username.
  new_username: Option<String>,
  
}



impl Object for ChatEventUsernameChanged {}
impl RObject for ChatEventUsernameChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventUsernameChanged" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventUsernameChanged }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventUsernameChanged {}


impl ChatEventUsernameChanged {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventUsernameChanged".to_string(),
      old_username: None,
      new_username: None,
      
    }
  }
  
  pub fn old_username(&self) -> Option<String> { self.old_username.clone() }
  #[doc(hidden)] pub fn _set_old_username(&mut self, old_username: String) -> &mut Self { self.old_username = Some(old_username); self }
  
  pub fn new_username(&self) -> Option<String> { self.new_username.clone() }
  #[doc(hidden)] pub fn _set_new_username(&mut self, new_username: String) -> &mut Self { self.new_username = Some(new_username); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



