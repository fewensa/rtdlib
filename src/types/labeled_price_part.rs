
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Portion of the price of a product (e.g., "delivery cost", "tax amount"). 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabeledPricePart {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // labeledPricePart
  /// Label for this portion of the product price.
  label: Option<String>,
  /// Currency amount in minimal quantity of the currency.
  amount: Option<i64>,
  
}



impl Object for LabeledPricePart {}
impl RObject for LabeledPricePart {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "labeledPricePart" }
  fn td_type(&self) -> RTDType { RTDType::LabeledPricePart }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl LabeledPricePart {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "labeledPricePart".to_string(),
      label: None,
      amount: None,
      
    }
  }
  
  pub fn label(&self) -> Option<String> { self.label.clone() }
  #[doc(hidden)] pub fn _set_label(&mut self, label: String) -> &mut Self { self.label = Some(label); self }
  
  pub fn amount(&self) -> Option<i64> { self.amount.clone() }
  #[doc(hidden)] pub fn _set_amount(&mut self, amount: i64) -> &mut Self { self.amount = Some(amount); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



