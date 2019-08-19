
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Checks the validity of an invite link for a chat and returns information about the corresponding chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckChatInviteLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkChatInviteLink
  /// Invite link to be checked; should begin with "https://t.me/joinchat/", "https://telegram.me/joinchat/", or "https://telegram.dog/joinchat/".
  invite_link: Option<String>,
  
}



impl Object for CheckChatInviteLink {}
impl RObject for CheckChatInviteLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChatInviteLink" }
  fn td_type(&self) -> RTDType { RTDType::CheckChatInviteLink }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CheckChatInviteLink {}


impl CheckChatInviteLink {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkChatInviteLink".to_string(),
      invite_link: None,
      
    }
  }
  
  pub fn invite_link(&self) -> Option<String> { self.invite_link.clone() }
  #[doc(hidden)] pub fn _set_invite_link(&mut self, invite_link: String) -> &mut Self { self.invite_link = Some(invite_link); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



