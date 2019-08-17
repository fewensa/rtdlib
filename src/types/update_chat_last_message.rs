
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The last message of a chat was changed. If last_message is null then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatLastMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateChatLastMessage
  /// Chat identifier.
  chat_id: Option<i64>,
  /// The new last message in the chat; may be null.
  last_message: Option<Message>,
  /// New value of the chat order.
  order: Option<String>,
  
}



impl Object for UpdateChatLastMessage {}
impl RObject for UpdateChatLastMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatLastMessage" }
  fn td_type(&self) -> RTDType { RTDType::UpdateChatLastMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateChatLastMessage {}


impl UpdateChatLastMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateChatLastMessage".to_string(),
      chat_id: None,
      last_message: None,
      order: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn last_message(&self) -> Option<Message> { self.last_message.clone() }
  #[doc(hidden)] pub fn _set_last_message(&mut self, last_message: Message) -> &mut Self { self.last_message = Some(last_message); self }
  
  pub fn order(&self) -> Option<String> { self.order.clone() }
  #[doc(hidden)] pub fn _set_order(&mut self, order: String) -> &mut Self { self.order = Some(order); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



