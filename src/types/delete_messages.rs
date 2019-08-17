
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Deletes messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deleteMessages
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Identifiers of the messages to be deleted.
  message_ids: Option<Vec<i64>>,
  /// Pass true to try to delete messages for all chat members. Always true for supergroups, channels and secret chats.
  revoke: Option<bool>,
  
}



impl Object for DeleteMessages {}
impl RObject for DeleteMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteMessages" }
  fn td_type(&self) -> RTDType { RTDType::DeleteMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DeleteMessages {}


impl DeleteMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deleteMessages".to_string(),
      chat_id: None,
      message_ids: None,
      revoke: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_ids(&self) -> Option<Vec<i64>> { self.message_ids.clone() }
  #[doc(hidden)] pub fn _set_message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self { self.message_ids = Some(message_ids); self }
  
  pub fn revoke(&self) -> Option<bool> { self.revoke.clone() }
  #[doc(hidden)] pub fn _set_revoke(&mut self, revoke: bool) -> &mut Self { self.revoke = Some(revoke); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



