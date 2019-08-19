
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns an invoice payment form. This method should be called when the user presses inlineKeyboardButtonBuy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaymentForm {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getPaymentForm
  /// Chat identifier of the Invoice message.
  chat_id: Option<i64>,
  /// Message identifier.
  message_id: Option<i64>,
  
}



impl Object for GetPaymentForm {}
impl RObject for GetPaymentForm {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPaymentForm" }
  fn td_type(&self) -> RTDType { RTDType::GetPaymentForm }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetPaymentForm {}


impl GetPaymentForm {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getPaymentForm".to_string(),
      chat_id: None,
      message_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



