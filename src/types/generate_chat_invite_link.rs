
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Generates a new invite link for a chat; the previously generated link is revoked. Available for basic groups, supergroups, and channels. In basic groups this can be called only by the group's creator; in supergroups and channels this requires appropriate administrator rights.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateChatInviteLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // generateChatInviteLink
  /// Chat identifier.
  chat_id: Option<i64>,
  
}



impl Object for GenerateChatInviteLink {}
impl RObject for GenerateChatInviteLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "generateChatInviteLink" }
  fn td_type(&self) -> RTDType { RTDType::GenerateChatInviteLink }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GenerateChatInviteLink {}


impl GenerateChatInviteLink {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "generateChatInviteLink".to_string(),
      chat_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



