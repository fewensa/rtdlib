
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Toggles sender signatures messages sent in a channel; requires appropriate administrator rights in the channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToggleSupergroupSignMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // toggleSupergroupSignMessages
  /// Identifier of the channel.
  supergroup_id: Option<i32>,
  /// New value of sign_messages.
  sign_messages: Option<bool>,
  
}



impl Object for ToggleSupergroupSignMessages {}
impl RObject for ToggleSupergroupSignMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "toggleSupergroupSignMessages" }
  fn td_type(&self) -> RTDType { RTDType::ToggleSupergroupSignMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ToggleSupergroupSignMessages {}


impl ToggleSupergroupSignMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "toggleSupergroupSignMessages".to_string(),
      supergroup_id: None,
      sign_messages: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn sign_messages(&self) -> Option<bool> { self.sign_messages.clone() }
  #[doc(hidden)] pub fn _set_sign_messages(&mut self, sign_messages: bool) -> &mut Self { self.sign_messages = Some(sign_messages); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



