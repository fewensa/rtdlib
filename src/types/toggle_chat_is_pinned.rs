
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the pinned state of a chat. You can pin up to GetOption("pinned_chat_count_max") non-secret chats and the same number of secret chats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToggleChatIsPinned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // toggleChatIsPinned
  /// Chat identifier.
  chat_id: Option<i64>,
  /// New value of is_pinned.
  is_pinned: Option<bool>,
  
}



impl Object for ToggleChatIsPinned {}
impl RObject for ToggleChatIsPinned {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "toggleChatIsPinned" }
  fn td_type(&self) -> RTDType { RTDType::ToggleChatIsPinned }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ToggleChatIsPinned {}


impl ToggleChatIsPinned {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "toggleChatIsPinned".to_string(),
      chat_id: None,
      is_pinned: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



