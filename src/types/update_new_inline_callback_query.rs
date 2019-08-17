
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new incoming callback query from a message sent via a bot; for bots only. 
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNewInlineCallbackQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateNewInlineCallbackQuery
  /// Unique query identifier.
  id: Option<i64>,
  /// Identifier of the user who sent the query.
  sender_user_id: Option<i32>,
  /// Identifier of the inline message, from which the query originated.
  inline_message_id: Option<String>,
  /// An identifier uniquely corresponding to the chat a message was sent to.
  chat_instance: Option<i64>,
  /// Query payload.
  payload: Option<Box<CallbackQueryPayload>>,
  
}


impl Clone for UpdateNewInlineCallbackQuery {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UpdateNewInlineCallbackQuery {}
impl RObject for UpdateNewInlineCallbackQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewInlineCallbackQuery" }
  fn td_type(&self) -> RTDType { RTDType::UpdateNewInlineCallbackQuery }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateNewInlineCallbackQuery {}


impl UpdateNewInlineCallbackQuery {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateNewInlineCallbackQuery".to_string(),
      id: None,
      sender_user_id: None,
      inline_message_id: None,
      chat_instance: None,
      payload: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn sender_user_id(&self) -> Option<i32> { self.sender_user_id.clone() }
  #[doc(hidden)] pub fn _set_sender_user_id(&mut self, sender_user_id: i32) -> &mut Self { self.sender_user_id = Some(sender_user_id); self }
  
  pub fn inline_message_id(&self) -> Option<String> { self.inline_message_id.clone() }
  #[doc(hidden)] pub fn _set_inline_message_id(&mut self, inline_message_id: String) -> &mut Self { self.inline_message_id = Some(inline_message_id); self }
  
  pub fn chat_instance(&self) -> Option<i64> { self.chat_instance.clone() }
  #[doc(hidden)] pub fn _set_chat_instance(&mut self, chat_instance: i64) -> &mut Self { self.chat_instance = Some(chat_instance); self }
  
  pub fn payload(&self) -> Option<Box<CallbackQueryPayload>> { self.payload.clone() }
  #[doc(hidden)] pub fn _set_payload(&mut self, payload: Box<CallbackQueryPayload>) -> &mut Self { self.payload = Some(payload); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



