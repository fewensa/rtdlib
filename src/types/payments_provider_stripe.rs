
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Stripe payment provider. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentsProviderStripe {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // paymentsProviderStripe
  /// Stripe API publishable key.
  publishable_key: Option<String>,
  /// True, if the user country must be provided.
  need_country: Option<bool>,
  /// True, if the user ZIP/postal code must be provided.
  need_postal_code: Option<bool>,
  /// True, if the cardholder name must be provided.
  need_cardholder_name: Option<bool>,
  
}



impl Object for PaymentsProviderStripe {}
impl RObject for PaymentsProviderStripe {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "paymentsProviderStripe" }
  fn td_type(&self) -> RTDType { RTDType::PaymentsProviderStripe }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PaymentsProviderStripe {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "paymentsProviderStripe".to_string(),
      publishable_key: None,
      need_country: None,
      need_postal_code: None,
      need_cardholder_name: None,
      
    }
  }
  
  pub fn publishable_key(&self) -> Option<String> { self.publishable_key.clone() }
  #[doc(hidden)] pub fn _set_publishable_key(&mut self, publishable_key: String) -> &mut Self { self.publishable_key = Some(publishable_key); self }
  
  pub fn need_country(&self) -> Option<bool> { self.need_country.clone() }
  #[doc(hidden)] pub fn _set_need_country(&mut self, need_country: bool) -> &mut Self { self.need_country = Some(need_country); self }
  
  pub fn need_postal_code(&self) -> Option<bool> { self.need_postal_code.clone() }
  #[doc(hidden)] pub fn _set_need_postal_code(&mut self, need_postal_code: bool) -> &mut Self { self.need_postal_code = Some(need_postal_code); self }
  
  pub fn need_cardholder_name(&self) -> Option<bool> { self.need_cardholder_name.clone() }
  #[doc(hidden)] pub fn _set_need_cardholder_name(&mut self, need_cardholder_name: bool) -> &mut Self { self.need_cardholder_name = Some(need_cardholder_name); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



