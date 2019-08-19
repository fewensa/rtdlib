
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The order of the chat in the chat list has changed. Instead of this update 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatOrder {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateChatOrder
  /// Chat identifier.
  chat_id: Option<i64>,
  /// New value of the order.
  order: Option<i64>,
  
}



impl Object for UpdateChatOrder {}
impl RObject for UpdateChatOrder {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatOrder" }
  fn td_type(&self) -> RTDType { RTDType::UpdateChatOrder }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateChatOrder {}


impl UpdateChatOrder {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateChatOrder".to_string(),
      chat_id: None,
      order: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn order(&self) -> Option<i64> { self.order.clone() }
  #[doc(hidden)] pub fn _set_order(&mut self, order: i64) -> &mut Self { self.order = Some(order); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



