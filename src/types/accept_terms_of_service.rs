
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Accepts Telegram terms of services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptTermsOfService {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // acceptTermsOfService
  /// Terms of service identifier.
  terms_of_service_id: Option<String>,
  
}



impl Object for AcceptTermsOfService {}
impl RObject for AcceptTermsOfService {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "acceptTermsOfService" }
  fn td_type(&self) -> RTDType { RTDType::AcceptTermsOfService }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AcceptTermsOfService {}


impl AcceptTermsOfService {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "acceptTermsOfService".to_string(),
      terms_of_service_id: None,
      
    }
  }
  
  pub fn terms_of_service_id(&self) -> Option<String> { self.terms_of_service_id.clone() }
  #[doc(hidden)] pub fn _set_terms_of_service_id(&mut self, terms_of_service_id: String) -> &mut Self { self.terms_of_service_id = Some(terms_of_service_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



