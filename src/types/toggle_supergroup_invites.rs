
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Toggles whether all members of a supergroup can add new members; requires appropriate administrator rights in the supergroup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToggleSupergroupInvites {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // toggleSupergroupInvites
  /// Identifier of the supergroup.
  supergroup_id: Option<i32>,
  /// New value of anyone_can_invite.
  anyone_can_invite: Option<bool>,
  
}



impl Object for ToggleSupergroupInvites {}
impl RObject for ToggleSupergroupInvites {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "toggleSupergroupInvites" }
  fn td_type(&self) -> RTDType { RTDType::ToggleSupergroupInvites }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ToggleSupergroupInvites {}


impl ToggleSupergroupInvites {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "toggleSupergroupInvites".to_string(),
      supergroup_id: None,
      anyone_can_invite: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn anyone_can_invite(&self) -> Option<bool> { self.anyone_can_invite.clone() }
  #[doc(hidden)] pub fn _set_anyone_can_invite(&mut self, anyone_can_invite: bool) -> &mut Self { self.anyone_can_invite = Some(anyone_can_invite); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



