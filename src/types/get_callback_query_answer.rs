
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends a callback query to a bot and returns an answer. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetCallbackQueryAnswer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getCallbackQueryAnswer
  /// Identifier of the chat with the message.
  chat_id: Option<i64>,
  /// Identifier of the message from which the query originated.
  message_id: Option<i64>,
  /// Query payload.
  payload: Option<Box<CallbackQueryPayload>>,
  
}


impl Clone for GetCallbackQueryAnswer {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for GetCallbackQueryAnswer {}
impl RObject for GetCallbackQueryAnswer {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getCallbackQueryAnswer" }
  fn td_type(&self) -> RTDType { RTDType::GetCallbackQueryAnswer }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetCallbackQueryAnswer {}


impl GetCallbackQueryAnswer {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getCallbackQueryAnswer".to_string(),
      chat_id: None,
      message_id: None,
      payload: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn payload(&self) -> Option<Box<CallbackQueryPayload>> { self.payload.clone() }
  #[doc(hidden)] pub fn _set_payload(&mut self, payload: Box<CallbackQueryPayload>) -> &mut Self { self.payload = Some(payload); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



