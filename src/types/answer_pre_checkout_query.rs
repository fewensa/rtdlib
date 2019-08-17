
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sets the result of a pre-checkout query; for bots only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerPreCheckoutQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // answerPreCheckoutQuery
  /// Identifier of the pre-checkout query.
  pre_checkout_query_id: Option<i64>,
  /// An error message, empty on success.
  error_message: Option<String>,
  
}



impl Object for AnswerPreCheckoutQuery {}
impl RObject for AnswerPreCheckoutQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "answerPreCheckoutQuery" }
  fn td_type(&self) -> RTDType { RTDType::AnswerPreCheckoutQuery }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AnswerPreCheckoutQuery {}


impl AnswerPreCheckoutQuery {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "answerPreCheckoutQuery".to_string(),
      pre_checkout_query_id: None,
      error_message: None,
      
    }
  }
  
  pub fn pre_checkout_query_id(&self) -> Option<i64> { self.pre_checkout_query_id.clone() }
  #[doc(hidden)] pub fn _set_pre_checkout_query_id(&mut self, pre_checkout_query_id: i64) -> &mut Self { self.pre_checkout_query_id = Some(pre_checkout_query_id); self }
  
  pub fn error_message(&self) -> Option<String> { self.error_message.clone() }
  #[doc(hidden)] pub fn _set_error_message(&mut self, error_message: String) -> &mut Self { self.error_message = Some(error_message); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



