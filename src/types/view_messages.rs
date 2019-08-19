
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Informs TDLib that messages are being viewed by the user. Many useful activities depend on whether the messages are currently being viewed or not (e.g., marking messages as read, incrementing a view counter, updating a view counter, removing deleted messages in supergroups and channels).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // viewMessages
  /// Chat identifier.
  chat_id: Option<i64>,
  /// The identifiers of the messages being viewed.
  message_ids: Option<Vec<i64>>,
  /// True, if messages in closed chats should be marked as read.
  force_read: Option<bool>,
  
}



impl Object for ViewMessages {}
impl RObject for ViewMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "viewMessages" }
  fn td_type(&self) -> RTDType { RTDType::ViewMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ViewMessages {}


impl ViewMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "viewMessages".to_string(),
      chat_id: None,
      message_ids: None,
      force_read: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_ids(&self) -> Option<Vec<i64>> { self.message_ids.clone() }
  #[doc(hidden)] pub fn _set_message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self { self.message_ids = Some(message_ids); self }
  
  pub fn force_read(&self) -> Option<bool> { self.force_read.clone() }
  #[doc(hidden)] pub fn _set_force_read(&mut self, force_read: bool) -> &mut Self { self.force_read = Some(force_read); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



