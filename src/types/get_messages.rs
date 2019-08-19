
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about messages. If a message is not found, returns null on the corresponding position of the result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getMessages
  /// Identifier of the chat the messages belong to.
  chat_id: Option<i64>,
  /// Identifiers of the messages to get.
  message_ids: Option<Vec<i64>>,
  
}



impl Object for GetMessages {}
impl RObject for GetMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getMessages" }
  fn td_type(&self) -> RTDType { RTDType::GetMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetMessages {}


impl GetMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getMessages".to_string(),
      chat_id: None,
      message_ids: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_ids(&self) -> Option<Vec<i64>> { self.message_ids.clone() }
  #[doc(hidden)] pub fn _set_message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self { self.message_ids = Some(message_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



