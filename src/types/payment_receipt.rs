
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about a successful payment. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentReceipt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // paymentReceipt
  /// Point in time (Unix timestamp) when the payment was made.
  date: Option<i32>,
  /// User identifier of the payment provider bot.
  payments_provider_user_id: Option<i32>,
  /// Contains information about the invoice.
  invoice: Option<Invoice>,
  /// Contains order information; may be null.
  order_info: Option<OrderInfo>,
  /// Chosen shipping option; may be null.
  shipping_option: Option<ShippingOption>,
  /// Title of the saved credentials.
  credentials_title: Option<String>,
  
}



impl Object for PaymentReceipt {}
impl RObject for PaymentReceipt {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "paymentReceipt" }
  fn td_type(&self) -> RTDType { RTDType::PaymentReceipt }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PaymentReceipt {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "paymentReceipt".to_string(),
      date: None,
      payments_provider_user_id: None,
      invoice: None,
      order_info: None,
      shipping_option: None,
      credentials_title: None,
      
    }
  }
  
  pub fn date(&self) -> Option<i32> { self.date.clone() }
  #[doc(hidden)] pub fn _set_date(&mut self, date: i32) -> &mut Self { self.date = Some(date); self }
  
  pub fn payments_provider_user_id(&self) -> Option<i32> { self.payments_provider_user_id.clone() }
  #[doc(hidden)] pub fn _set_payments_provider_user_id(&mut self, payments_provider_user_id: i32) -> &mut Self { self.payments_provider_user_id = Some(payments_provider_user_id); self }
  
  pub fn invoice(&self) -> Option<Invoice> { self.invoice.clone() }
  #[doc(hidden)] pub fn _set_invoice(&mut self, invoice: Invoice) -> &mut Self { self.invoice = Some(invoice); self }
  
  pub fn order_info(&self) -> Option<OrderInfo> { self.order_info.clone() }
  #[doc(hidden)] pub fn _set_order_info(&mut self, order_info: OrderInfo) -> &mut Self { self.order_info = Some(order_info); self }
  
  pub fn shipping_option(&self) -> Option<ShippingOption> { self.shipping_option.clone() }
  #[doc(hidden)] pub fn _set_shipping_option(&mut self, shipping_option: ShippingOption) -> &mut Self { self.shipping_option = Some(shipping_option); self }
  
  pub fn credentials_title(&self) -> Option<String> { self.credentials_title.clone() }
  #[doc(hidden)] pub fn _set_credentials_title(&mut self, credentials_title: String) -> &mut Self { self.credentials_title = Some(credentials_title); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



