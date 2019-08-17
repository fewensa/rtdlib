
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sets the result of a shipping query; for bots only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerShippingQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // answerShippingQuery
  /// Identifier of the shipping query.
  shipping_query_id: Option<i64>,
  /// Available shipping options.
  shipping_options: Option<Vec<ShippingOption>>,
  /// An error message, empty on success.
  error_message: Option<String>,
  
}



impl Object for AnswerShippingQuery {}
impl RObject for AnswerShippingQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "answerShippingQuery" }
  fn td_type(&self) -> RTDType { RTDType::AnswerShippingQuery }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AnswerShippingQuery {}


impl AnswerShippingQuery {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "answerShippingQuery".to_string(),
      shipping_query_id: None,
      shipping_options: None,
      error_message: None,
      
    }
  }
  
  pub fn shipping_query_id(&self) -> Option<i64> { self.shipping_query_id.clone() }
  #[doc(hidden)] pub fn _set_shipping_query_id(&mut self, shipping_query_id: i64) -> &mut Self { self.shipping_query_id = Some(shipping_query_id); self }
  
  pub fn shipping_options(&self) -> Option<Vec<ShippingOption>> { self.shipping_options.clone() }
  #[doc(hidden)] pub fn _set_shipping_options(&mut self, shipping_options: Vec<ShippingOption>) -> &mut Self { self.shipping_options = Some(shipping_options); self }
  
  pub fn error_message(&self) -> Option<String> { self.error_message.clone() }
  #[doc(hidden)] pub fn _set_error_message(&mut self, error_message: String) -> &mut Self { self.error_message = Some(error_message); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



