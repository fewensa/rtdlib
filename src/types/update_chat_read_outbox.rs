
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Outgoing messages were read. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatReadOutbox {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateChatReadOutbox
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Identifier of last read outgoing message.
  last_read_outbox_message_id: Option<i64>,
  
}



impl Object for UpdateChatReadOutbox {}
impl RObject for UpdateChatReadOutbox {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatReadOutbox" }
  fn td_type(&self) -> RTDType { RTDType::UpdateChatReadOutbox }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateChatReadOutbox {}


impl UpdateChatReadOutbox {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateChatReadOutbox".to_string(),
      chat_id: None,
      last_read_outbox_message_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn last_read_outbox_message_id(&self) -> Option<i64> { self.last_read_outbox_message_id.clone() }
  #[doc(hidden)] pub fn _set_last_read_outbox_message_id(&mut self, last_read_outbox_message_id: i64) -> &mut Self { self.last_read_outbox_message_id = Some(last_read_outbox_message_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



