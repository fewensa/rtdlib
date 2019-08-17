
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// New terms of service must be accepted by the user. If the terms of service are declined, then the 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTermsOfService {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateTermsOfService
  /// Identifier of the terms of service.
  terms_of_service_id: Option<String>,
  /// The new terms of service.
  terms_of_service: Option<TermsOfService>,
  
}



impl Object for UpdateTermsOfService {}
impl RObject for UpdateTermsOfService {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateTermsOfService" }
  fn td_type(&self) -> RTDType { RTDType::UpdateTermsOfService }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateTermsOfService {}


impl UpdateTermsOfService {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateTermsOfService".to_string(),
      terms_of_service_id: None,
      terms_of_service: None,
      
    }
  }
  
  pub fn terms_of_service_id(&self) -> Option<String> { self.terms_of_service_id.clone() }
  #[doc(hidden)] pub fn _set_terms_of_service_id(&mut self, terms_of_service_id: String) -> &mut Self { self.terms_of_service_id = Some(terms_of_service_id); self }
  
  pub fn terms_of_service(&self) -> Option<TermsOfService> { self.terms_of_service.clone() }
  #[doc(hidden)] pub fn _set_terms_of_service(&mut self, terms_of_service: TermsOfService) -> &mut Self { self.terms_of_service = Some(terms_of_service); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



