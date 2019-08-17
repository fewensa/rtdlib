
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update shouldn't be applied. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatDraftMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateChatDraftMessage
  /// Chat identifier.
  chat_id: Option<i64>,
  /// The new draft message; may be null.
  draft_message: Option<DraftMessage>,
  /// New value of the chat order.
  order: Option<i64>,
  
}



impl Object for UpdateChatDraftMessage {}
impl RObject for UpdateChatDraftMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatDraftMessage" }
  fn td_type(&self) -> RTDType { RTDType::UpdateChatDraftMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateChatDraftMessage {}


impl UpdateChatDraftMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateChatDraftMessage".to_string(),
      chat_id: None,
      draft_message: None,
      order: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn draft_message(&self) -> Option<DraftMessage> { self.draft_message.clone() }
  #[doc(hidden)] pub fn _set_draft_message(&mut self, draft_message: DraftMessage) -> &mut Self { self.draft_message = Some(draft_message); self }
  
  pub fn order(&self) -> Option<i64> { self.order.clone() }
  #[doc(hidden)] pub fn _set_order(&mut self, order: i64) -> &mut Self { self.order = Some(order); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



