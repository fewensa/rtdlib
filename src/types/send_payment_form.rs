
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends a filled-out payment form to the bot for final verification.
#[derive(Debug, Serialize, Deserialize)]
pub struct SendPaymentForm {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sendPaymentForm
  /// Chat identifier of the Invoice message.
  chat_id: Option<i64>,
  /// Message identifier.
  message_id: Option<i64>,
  /// Identifier returned by ValidateOrderInfo, or an empty string.
  order_info_id: Option<String>,
  /// Identifier of a chosen shipping option, if applicable.
  shipping_option_id: Option<String>,
  /// The credentials chosen by user for payment.
  credentials: Option<Box<InputCredentials>>,
  
}


impl Clone for SendPaymentForm {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SendPaymentForm {}
impl RObject for SendPaymentForm {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendPaymentForm" }
  fn td_type(&self) -> RTDType { RTDType::SendPaymentForm }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SendPaymentForm {}


impl SendPaymentForm {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sendPaymentForm".to_string(),
      chat_id: None,
      message_id: None,
      order_info_id: None,
      shipping_option_id: None,
      credentials: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn order_info_id(&self) -> Option<String> { self.order_info_id.clone() }
  #[doc(hidden)] pub fn _set_order_info_id(&mut self, order_info_id: String) -> &mut Self { self.order_info_id = Some(order_info_id); self }
  
  pub fn shipping_option_id(&self) -> Option<String> { self.shipping_option_id.clone() }
  #[doc(hidden)] pub fn _set_shipping_option_id(&mut self, shipping_option_id: String) -> &mut Self { self.shipping_option_id = Some(shipping_option_id); self }
  
  pub fn credentials(&self) -> Option<Box<InputCredentials>> { self.credentials.clone() }
  #[doc(hidden)] pub fn _set_credentials(&mut self, credentials: Box<InputCredentials>) -> &mut Self { self.credentials = Some(credentials); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



