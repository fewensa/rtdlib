
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a chat invite link. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatInviteLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatInviteLink
  /// Chat invite link.
  invite_link: Option<String>,
  
}



impl Object for ChatInviteLink {}
impl RObject for ChatInviteLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatInviteLink" }
  fn td_type(&self) -> RTDType { RTDType::ChatInviteLink }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ChatInviteLink {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatInviteLink".to_string(),
      invite_link: None,
      
    }
  }
  
  pub fn invite_link(&self) -> Option<String> { self.invite_link.clone() }
  #[doc(hidden)] pub fn _set_invite_link(&mut self, invite_link: String) -> &mut Self { self.invite_link = Some(invite_link); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



