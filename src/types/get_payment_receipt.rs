
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about a successful payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaymentReceipt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getPaymentReceipt
  /// Chat identifier of the PaymentSuccessful message.
  chat_id: Option<i64>,
  /// Message identifier.
  message_id: Option<i64>,
  
}



impl Object for GetPaymentReceipt {}
impl RObject for GetPaymentReceipt {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPaymentReceipt" }
  fn td_type(&self) -> RTDType { RTDType::GetPaymentReceipt }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetPaymentReceipt {}


impl GetPaymentReceipt {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getPaymentReceipt".to_string(),
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



