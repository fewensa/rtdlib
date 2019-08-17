
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new incoming pre-checkout query; for bots only. Contains full information about a checkout. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNewPreCheckoutQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateNewPreCheckoutQuery
  /// Unique query identifier.
  id: Option<i64>,
  /// Identifier of the user who sent the query.
  sender_user_id: Option<i32>,
  /// Currency for the product price.
  currency: Option<String>,
  /// Total price for the product, in the minimal quantity of the currency.
  total_amount: Option<i64>,
  /// Invoice payload.
  invoice_payload: Option<String>,
  /// Identifier of a shipping option chosen by the user; may be empty if not applicable.
  shipping_option_id: Option<String>,
  /// Information about the order; may be null.
  order_info: Option<OrderInfo>,
  
}



impl Object for UpdateNewPreCheckoutQuery {}
impl RObject for UpdateNewPreCheckoutQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewPreCheckoutQuery" }
  fn td_type(&self) -> RTDType { RTDType::UpdateNewPreCheckoutQuery }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateNewPreCheckoutQuery {}


impl UpdateNewPreCheckoutQuery {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateNewPreCheckoutQuery".to_string(),
      id: None,
      sender_user_id: None,
      currency: None,
      total_amount: None,
      invoice_payload: None,
      shipping_option_id: None,
      order_info: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn sender_user_id(&self) -> Option<i32> { self.sender_user_id.clone() }
  #[doc(hidden)] pub fn _set_sender_user_id(&mut self, sender_user_id: i32) -> &mut Self { self.sender_user_id = Some(sender_user_id); self }
  
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
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



