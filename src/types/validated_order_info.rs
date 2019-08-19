
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a temporary identifier of validated order information, which is stored for one hour. Also contains the available shipping options. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedOrderInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // validatedOrderInfo
  /// Temporary identifier of the order information.
  order_info_id: Option<String>,
  /// Available shipping options.
  shipping_options: Option<Vec<ShippingOption>>,
  
}



impl Object for ValidatedOrderInfo {}
impl RObject for ValidatedOrderInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "validatedOrderInfo" }
  fn td_type(&self) -> RTDType { RTDType::ValidatedOrderInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ValidatedOrderInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "validatedOrderInfo".to_string(),
      order_info_id: None,
      shipping_options: None,
      
    }
  }
  
  pub fn order_info_id(&self) -> Option<String> { self.order_info_id.clone() }
  #[doc(hidden)] pub fn _set_order_info_id(&mut self, order_info_id: String) -> &mut Self { self.order_info_id = Some(order_info_id); self }
  
  pub fn shipping_options(&self) -> Option<Vec<ShippingOption>> { self.shipping_options.clone() }
  #[doc(hidden)] pub fn _set_shipping_options(&mut self, shipping_options: Vec<ShippingOption>) -> &mut Self { self.shipping_options = Some(shipping_options); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



