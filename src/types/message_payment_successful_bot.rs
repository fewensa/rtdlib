
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A payment has been completed; for bots only. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePaymentSuccessfulBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messagePaymentSuccessfulBot
  /// Identifier of the message with the corresponding invoice; can be an identifier of a deleted message.
  invoice_message_id: Option<i64>,
  /// Currency for price of the product.
  currency: Option<String>,
  /// Total price for the product, in the minimal quantity of the currency.
  total_amount: Option<i64>,
  /// Invoice payload.
  invoice_payload: Option<String>,
  /// Identifier of the shipping option chosen by the user; may be empty if not applicable.
  shipping_option_id: Option<String>,
  /// Information about the order; may be null.
  order_info: Option<OrderInfo>,
  /// Telegram payment identifier.
  telegram_payment_charge_id: Option<String>,
  /// Provider payment identifier.
  provider_payment_charge_id: Option<String>,
  
}



impl Object for MessagePaymentSuccessfulBot {}
impl RObject for MessagePaymentSuccessfulBot {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePaymentSuccessfulBot" }
  fn td_type(&self) -> RTDType { RTDType::MessagePaymentSuccessfulBot }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessagePaymentSuccessfulBot {}


impl MessagePaymentSuccessfulBot {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messagePaymentSuccessfulBot".to_string(),
      invoice_message_id: None,
      currency: None,
      total_amount: None,
      invoice_payload: None,
      shipping_option_id: None,
      order_info: None,
      telegram_payment_charge_id: None,
      provider_payment_charge_id: None,
      
    }
  }
  
  pub fn invoice_message_id(&self) -> Option<i64> { self.invoice_message_id.clone() }
  #[doc(hidden)] pub fn _set_invoice_message_id(&mut self, invoice_message_id: i64) -> &mut Self { self.invoice_message_id = Some(invoice_message_id); self }
  
  pub fn currency(&self) -> Option<String> { self.currency.clone() }
  #[doc(hidden)] pub fn _set_currency(&mut self, currency: String) -> &mut Self { self.currency = Some(currency); self }
  
  pub fn total_amount(&self) -> Option<i64> { self.total_amount.clone() }
  #[doc(hidden)] pub fn _set_total_amount(&mut self, total_amount: i64) -> &mut Self { self.total_amount = Some(total_amount); self }
  
  pub fn invoice_payload(&self) -> Option<String> { self.invoice_payload.clone() }
  #[doc(hidden)] pub fn _set_invoice_payload(&mut self, invoice_payload: String) -> &mut Self { self.invoice_payload = Some(invoice_payload); self }
  
  pub fn shipping_option_id(&self) -> Option<String> { self.shipping_option_id.clone() }
  #[doc(hidden)] pub fn _set_shipping_option_id(&mut self, shipping_option_id: String) -> &mut Self { self.shipping_option_id = Some(shipping_option_id); self }
  
  pub fn order_info(&self) -> Option<OrderInfo> { self.order_info.clone() }
  #[doc(hidden)] pub fn _set_order_info(&mut self, order_info: OrderInfo) -> &mut Self { self.order_info = Some(order_info); self }
  
  pub fn telegram_payment_charge_id(&self) -> Option<String> { self.telegram_payment_charge_id.clone() }
  #[doc(hidden)] pub fn _set_telegram_payment_charge_id(&mut self, telegram_payment_charge_id: String) -> &mut Self { self.telegram_payment_charge_id = Some(telegram_payment_charge_id); self }
  
  pub fn provider_payment_charge_id(&self) -> Option<String> { self.provider_payment_charge_id.clone() }
  #[doc(hidden)] pub fn _set_provider_payment_charge_id(&mut self, provider_payment_charge_id: String) -> &mut Self { self.provider_payment_charge_id = Some(provider_payment_charge_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



