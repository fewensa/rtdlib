
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Validates the order information provided by a user and returns the available shipping options for a flexible invoice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOrderInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // validateOrderInfo
  /// Chat identifier of the Invoice message.
  chat_id: Option<i64>,
  /// Message identifier.
  message_id: Option<i64>,
  /// The order information, provided by the user.
  order_info: Option<OrderInfo>,
  /// True, if the order information can be saved.
  allow_save: Option<bool>,
  
}



impl Object for ValidateOrderInfo {}
impl RObject for ValidateOrderInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "validateOrderInfo" }
  fn td_type(&self) -> RTDType { RTDType::ValidateOrderInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ValidateOrderInfo {}


impl ValidateOrderInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "validateOrderInfo".to_string(),
      chat_id: None,
      message_id: None,
      order_info: None,
      allow_save: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn order_info(&self) -> Option<OrderInfo> { self.order_info.clone() }
  #[doc(hidden)] pub fn _set_order_info(&mut self, order_info: OrderInfo) -> &mut Self { self.order_info = Some(order_info); self }
  
  pub fn allow_save(&self) -> Option<bool> { self.allow_save.clone() }
  #[doc(hidden)] pub fn _set_allow_save(&mut self, allow_save: bool) -> &mut Self { self.allow_save = Some(allow_save); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



