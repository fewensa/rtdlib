
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new incoming callback query; for bots only. 
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNewCallbackQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateNewCallbackQuery
  /// Unique query identifier.
  id: Option<i64>,
  /// Identifier of the user who sent the query.
  sender_user_id: Option<i32>,
  /// Identifier of the chat, in which the query was sent.
  chat_id: Option<i64>,
  /// Identifier of the message, from which the query originated.
  message_id: Option<i64>,
  /// Identifier that uniquely corresponds to the chat to which the message was sent.
  chat_instance: Option<i64>,
  /// Query payload.
  payload: Option<Box<CallbackQueryPayload>>,
  
}


impl Clone for UpdateNewCallbackQuery {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UpdateNewCallbackQuery {}
impl RObject for UpdateNewCallbackQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewCallbackQuery" }
  fn td_type(&self) -> RTDType { RTDType::UpdateNewCallbackQuery }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateNewCallbackQuery {}


impl UpdateNewCallbackQuery {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateNewCallbackQuery".to_string(),
      id: None,
      sender_user_id: None,
      chat_id: None,
      message_id: None,
      chat_instance: None,
      payload: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn sender_user_id(&self) -> Option<i32> { self.sender_user_id.clone() }
  #[doc(hidden)] pub fn _set_sender_user_id(&mut self, sender_user_id: i32) -> &mut Self { self.sender_user_id = Some(sender_user_id); self }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn chat_instance(&self) -> Option<i64> { self.chat_instance.clone() }
  #[doc(hidden)] pub fn _set_chat_instance(&mut self, chat_instance: i64) -> &mut Self { self.chat_instance = Some(chat_instance); self }
  
  pub fn payload(&self) -> Option<Box<CallbackQueryPayload>> { self.payload.clone() }
  #[doc(hidden)] pub fn _set_payload(&mut self, payload: Box<CallbackQueryPayload>) -> &mut Self { self.payload = Some(payload); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



