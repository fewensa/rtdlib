
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with an invoice from a bot. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageInvoice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageInvoice
  /// Product title.
  title: Option<String>,
  /// Product description.
  description: Option<String>,
  /// Product photo; may be null.
  photo: Option<Photo>,
  /// Currency for the product price.
  currency: Option<String>,
  /// Product total price in the minimal quantity of the currency.
  total_amount: Option<i64>,
  /// Unique invoice bot start_parameter. To share an invoice use the URL https://t.me/{bot_username}?start={start_parameter}.
  start_parameter: Option<String>,
  /// True, if the invoice is a test invoice.
  is_test: Option<bool>,
  /// True, if the shipping address should be specified.
  need_shipping_address: Option<bool>,
  /// The identifier of the message with the receipt, after the product has been purchased.
  receipt_message_id: Option<i64>,
  
}



impl Object for MessageInvoice {}
impl RObject for MessageInvoice {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageInvoice" }
  fn td_type(&self) -> RTDType { RTDType::MessageInvoice }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageInvoice {}


impl MessageInvoice {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageInvoice".to_string(),
      title: None,
      description: None,
      photo: None,
      currency: None,
      total_amount: None,
      start_parameter: None,
      is_test: None,
      need_shipping_address: None,
      receipt_message_id: None,
      
    }
  }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn photo(&self) -> Option<Photo> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: Photo) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn currency(&self) -> Option<String> { self.currency.clone() }
  #[doc(hidden)] pub fn _set_currency(&mut self, currency: String) -> &mut Self { self.currency = Some(currency); self }
  
  pub fn total_amount(&self) -> Option<i64> { self.total_amount.clone() }
  #[doc(hidden)] pub fn _set_total_amount(&mut self, total_amount: i64) -> &mut Self { self.total_amount = Some(total_amount); self }
  
  pub fn start_parameter(&self) -> Option<String> { self.start_parameter.clone() }
  #[doc(hidden)] pub fn _set_start_parameter(&mut self, start_parameter: String) -> &mut Self { self.start_parameter = Some(start_parameter); self }
  
  pub fn is_test(&self) -> Option<bool> { self.is_test.clone() }
  #[doc(hidden)] pub fn _set_is_test(&mut self, is_test: bool) -> &mut Self { self.is_test = Some(is_test); self }
  
  pub fn need_shipping_address(&self) -> Option<bool> { self.need_shipping_address.clone() }
  #[doc(hidden)] pub fn _set_need_shipping_address(&mut self, need_shipping_address: bool) -> &mut Self { self.need_shipping_address = Some(need_shipping_address); self }
  
  pub fn receipt_message_id(&self) -> Option<i64> { self.receipt_message_id.clone() }
  #[doc(hidden)] pub fn _set_receipt_message_id(&mut self, receipt_message_id: i64) -> &mut Self { self.receipt_message_id = Some(receipt_message_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



