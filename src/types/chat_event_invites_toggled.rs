
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The anyone_can_invite setting of a supergroup chat was toggled. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventInvitesToggled {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventInvitesToggled
  /// New value of anyone_can_invite.
  anyone_can_invite: Option<bool>,
  
}



impl Object for ChatEventInvitesToggled {}
impl RObject for ChatEventInvitesToggled {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventInvitesToggled" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventInvitesToggled }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventInvitesToggled {}


impl ChatEventInvitesToggled {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventInvitesToggled".to_string(),
      anyone_can_invite: None,
      
    }
  }
  
  pub fn anyone_can_invite(&self) -> Option<bool> { self.anyone_can_invite.clone() }
  #[doc(hidden)] pub fn _set_anyone_can_invite(&mut self, anyone_can_invite: bool) -> &mut Self { self.anyone_can_invite = Some(anyone_can_invite); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



