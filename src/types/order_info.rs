
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Order information. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // orderInfo
  /// Name of the user.
  name: Option<String>,
  /// Phone number of the user.
  phone_number: Option<String>,
  /// Email address of the user.
  email_address: Option<String>,
  /// Shipping address for this order; may be null.
  shipping_address: Option<Address>,
  
}



impl Object for OrderInfo {}
impl RObject for OrderInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "orderInfo" }
  fn td_type(&self) -> RTDType { RTDType::OrderInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl OrderInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "orderInfo".to_string(),
      name: None,
      phone_number: None,
      email_address: None,
      shipping_address: None,
      
    }
  }
  
  pub fn name(&self) -> Option<String> { self.name.clone() }
  #[doc(hidden)] pub fn _set_name(&mut self, name: String) -> &mut Self { self.name = Some(name); self }
  
  pub fn phone_number(&self) -> Option<String> { self.phone_number.clone() }
  #[doc(hidden)] pub fn _set_phone_number(&mut self, phone_number: String) -> &mut Self { self.phone_number = Some(phone_number); self }
  
  pub fn email_address(&self) -> Option<String> { self.email_address.clone() }
  #[doc(hidden)] pub fn _set_email_address(&mut self, email_address: String) -> &mut Self { self.email_address = Some(email_address); self }
  
  pub fn shipping_address(&self) -> Option<Address> { self.shipping_address.clone() }
  #[doc(hidden)] pub fn _set_shipping_address(&mut self, shipping_address: Address) -> &mut Self { self.shipping_address = Some(shipping_address); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



