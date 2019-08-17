
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains the result of a payment request. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResult {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // paymentResult
  /// True, if the payment request was successful; otherwise the verification_url will be not empty.
  success: Option<bool>,
  /// URL for additional payment credentials verification.
  verification_url: Option<String>,
  
}



impl Object for PaymentResult {}
impl RObject for PaymentResult {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "paymentResult" }
  fn td_type(&self) -> RTDType { RTDType::PaymentResult }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PaymentResult {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "paymentResult".to_string(),
      success: None,
      verification_url: None,
      
    }
  }
  
  pub fn success(&self) -> Option<bool> { self.success.clone() }
  #[doc(hidden)] pub fn _set_success(&mut self, success: bool) -> &mut Self { self.success = Some(success); self }
  
  pub fn verification_url(&self) -> Option<String> { self.verification_url.clone() }
  #[doc(hidden)] pub fn _set_verification_url(&mut self, verification_url: String) -> &mut Self { self.verification_url = Some(verification_url); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



