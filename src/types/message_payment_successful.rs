
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A payment has been completed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePaymentSuccessful {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messagePaymentSuccessful
  /// Identifier of the message with the corresponding invoice; can be an identifier of a deleted message.
  invoice_message_id: Option<i64>,
  /// Currency for the price of the product.
  currency: Option<String>,
  /// Total price for the product, in the minimal quantity of the currency.
  total_amount: Option<i64>,
  
}



impl Object for MessagePaymentSuccessful {}
impl RObject for MessagePaymentSuccessful {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePaymentSuccessful" }
  fn td_type(&self) -> RTDType { RTDType::MessagePaymentSuccessful }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessagePaymentSuccessful {}


impl MessagePaymentSuccessful {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messagePaymentSuccessful".to_string(),
      invoice_message_id: None,
      currency: None,
      total_amount: None,
      
    }
  }
  
  pub fn invoice_message_id(&self) -> Option<i64> { self.invoice_message_id.clone() }
  #[doc(hidden)] pub fn _set_invoice_message_id(&mut self, invoice_message_id: i64) -> &mut Self { self.invoice_message_id = Some(invoice_message_id); self }
  
  pub fn currency(&self) -> Option<String> { self.currency.clone() }
  #[doc(hidden)] pub fn _set_currency(&mut self, currency: String) -> &mut Self { self.currency = Some(currency); self }
  
  pub fn total_amount(&self) -> Option<i64> { self.total_amount.clone() }
  #[doc(hidden)] pub fn _set_total_amount(&mut self, total_amount: i64) -> &mut Self { self.total_amount = Some(total_amount); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



