
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Product invoice. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // invoice
  /// ISO 4217 currency code.
  currency: Option<String>,
  /// A list of objects used to calculate the total price of the product.
  price_parts: Option<Vec<LabeledPricePart>>,
  /// True, if the payment is a test payment.
  is_test: Option<bool>,
  /// True, if the user's name is needed for payment.
  need_name: Option<bool>,
  /// True, if the user's phone number is needed for payment.
  need_phone_number: Option<bool>,
  /// True, if the user's email address is needed for payment.
  need_email_address: Option<bool>,
  /// True, if the user's shipping address is needed for payment.
  need_shipping_address: Option<bool>,
  /// True, if the user's phone number will be sent to the provider.
  send_phone_number_to_provider: Option<bool>,
  /// True, if the user's email address will be sent to the provider.
  send_email_address_to_provider: Option<bool>,
  /// True, if the total price depends on the shipping method.
  is_flexible: Option<bool>,
  
}



impl Object for Invoice {}
impl RObject for Invoice {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "invoice" }
  fn td_type(&self) -> RTDType { RTDType::Invoice }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Invoice {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "invoice".to_string(),
      currency: None,
      price_parts: None,
      is_test: None,
      need_name: None,
      need_phone_number: None,
      need_email_address: None,
      need_shipping_address: None,
      send_phone_number_to_provider: None,
      send_email_address_to_provider: None,
      is_flexible: None,
      
    }
  }
  
  pub fn currency(&self) -> Option<String> { self.currency.clone() }
  #[doc(hidden)] pub fn _set_currency(&mut self, currency: String) -> &mut Self { self.currency = Some(currency); self }
  
  pub fn price_parts(&self) -> Option<Vec<LabeledPricePart>> { self.price_parts.clone() }
  #[doc(hidden)] pub fn _set_price_parts(&mut self, price_parts: Vec<LabeledPricePart>) -> &mut Self { self.price_parts = Some(price_parts); self }
  
  pub fn is_test(&self) -> Option<bool> { self.is_test.clone() }
  #[doc(hidden)] pub fn _set_is_test(&mut self, is_test: bool) -> &mut Self { self.is_test = Some(is_test); self }
  
  pub fn need_name(&self) -> Option<bool> { self.need_name.clone() }
  #[doc(hidden)] pub fn _set_need_name(&mut self, need_name: bool) -> &mut Self { self.need_name = Some(need_name); self }
  
  pub fn need_phone_number(&self) -> Option<bool> { self.need_phone_number.clone() }
  #[doc(hidden)] pub fn _set_need_phone_number(&mut self, need_phone_number: bool) -> &mut Self { self.need_phone_number = Some(need_phone_number); self }
  
  pub fn need_email_address(&self) -> Option<bool> { self.need_email_address.clone() }
  #[doc(hidden)] pub fn _set_need_email_address(&mut self, need_email_address: bool) -> &mut Self { self.need_email_address = Some(need_email_address); self }
  
  pub fn need_shipping_address(&self) -> Option<bool> { self.need_shipping_address.clone() }
  #[doc(hidden)] pub fn _set_need_shipping_address(&mut self, need_shipping_address: bool) -> &mut Self { self.need_shipping_address = Some(need_shipping_address); self }
  
  pub fn send_phone_number_to_provider(&self) -> Option<bool> { self.send_phone_number_to_provider.clone() }
  #[doc(hidden)] pub fn _set_send_phone_number_to_provider(&mut self, send_phone_number_to_provider: bool) -> &mut Self { self.send_phone_number_to_provider = Some(send_phone_number_to_provider); self }
  
  pub fn send_email_address_to_provider(&self) -> Option<bool> { self.send_email_address_to_provider.clone() }
  #[doc(hidden)] pub fn _set_send_email_address_to_provider(&mut self, send_email_address_to_provider: bool) -> &mut Self { self.send_email_address_to_provider = Some(send_email_address_to_provider); self }
  
  pub fn is_flexible(&self) -> Option<bool> { self.is_flexible.clone() }
  #[doc(hidden)] pub fn _set_is_flexible(&mut self, is_flexible: bool) -> &mut Self { self.is_flexible = Some(is_flexible); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



