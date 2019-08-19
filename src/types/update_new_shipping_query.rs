
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new incoming shipping query; for bots only. Only for invoices with flexible price. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNewShippingQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateNewShippingQuery
  /// Unique query identifier.
  id: Option<i64>,
  /// Identifier of the user who sent the query.
  sender_user_id: Option<i32>,
  /// Invoice payload.
  invoice_payload: Option<String>,
  /// User shipping address.
  shipping_address: Option<Address>,
  
}



impl Object for UpdateNewShippingQuery {}
impl RObject for UpdateNewShippingQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewShippingQuery" }
  fn td_type(&self) -> RTDType { RTDType::UpdateNewShippingQuery }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateNewShippingQuery {}


impl UpdateNewShippingQuery {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateNewShippingQuery".to_string(),
      id: None,
      sender_user_id: None,
      invoice_payload: None,
      shipping_address: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn sender_user_id(&self) -> Option<i32> { self.sender_user_id.clone() }
  #[doc(hidden)] pub fn _set_sender_user_id(&mut self, sender_user_id: i32) -> &mut Self { self.sender_user_id = Some(sender_user_id); self }
  
  pub fn invoice_payload(&self) -> Option<String> { self.invoice_payload.clone() }
  #[doc(hidden)] pub fn _set_invoice_payload(&mut self, invoice_payload: String) -> &mut Self { self.invoice_payload = Some(invoice_payload); self }
  
  pub fn shipping_address(&self) -> Option<Address> { self.shipping_address.clone() }
  #[doc(hidden)] pub fn _set_shipping_address(&mut self, shipping_address: Address) -> &mut Self { self.shipping_address = Some(shipping_address); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



