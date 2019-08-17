
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// One shipping option. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // shippingOption
  /// Shipping option identifier.
  id: Option<String>,
  /// Option title.
  title: Option<String>,
  /// A list of objects used to calculate the total shipping costs.
  price_parts: Option<Vec<LabeledPricePart>>,
  
}



impl Object for ShippingOption {}
impl RObject for ShippingOption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "shippingOption" }
  fn td_type(&self) -> RTDType { RTDType::ShippingOption }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ShippingOption {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "shippingOption".to_string(),
      id: None,
      title: None,
      price_parts: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn price_parts(&self) -> Option<Vec<LabeledPricePart>> { self.price_parts.clone() }
  #[doc(hidden)] pub fn _set_price_parts(&mut self, price_parts: Vec<LabeledPricePart>) -> &mut Self { self.price_parts = Some(price_parts); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



