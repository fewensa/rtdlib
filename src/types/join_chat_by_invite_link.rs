
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Uses an invite link to add the current user to the chat if possible. The new member will not be added until the chat state has been synchronized with the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinChatByInviteLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // joinChatByInviteLink
  /// Invite link to import; should begin with "https://t.me/joinchat/", "https://telegram.me/joinchat/", or "https://telegram.dog/joinchat/".
  invite_link: Option<String>,
  
}



impl Object for JoinChatByInviteLink {}
impl RObject for JoinChatByInviteLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "joinChatByInviteLink" }
  fn td_type(&self) -> RTDType { RTDType::JoinChatByInviteLink }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for JoinChatByInviteLink {}


impl JoinChatByInviteLink {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "joinChatByInviteLink".to_string(),
      invite_link: None,
      
    }
  }
  
  pub fn invite_link(&self) -> Option<String> { self.invite_link.clone() }
  #[doc(hidden)] pub fn _set_invite_link(&mut self, invite_link: String) -> &mut Self { self.invite_link = Some(invite_link); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



