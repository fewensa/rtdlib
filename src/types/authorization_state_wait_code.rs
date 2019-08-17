
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// TDLib needs the user's authentication code to finalize authorization. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateWaitCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // authorizationStateWaitCode
  /// True, if the user is already registered.
  is_registered: Option<bool>,
  /// Telegram terms of service, which should be accepted before user can continue registration; may be null.
  terms_of_service: Option<TermsOfService>,
  /// Information about the authorization code that was sent.
  code_info: Option<AuthenticationCodeInfo>,
  
}



impl Object for AuthorizationStateWaitCode {}
impl RObject for AuthorizationStateWaitCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitCode" }
  fn td_type(&self) -> RTDType { RTDType::AuthorizationStateWaitCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl AuthorizationState for AuthorizationStateWaitCode {}


impl AuthorizationStateWaitCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "authorizationStateWaitCode".to_string(),
      is_registered: None,
      terms_of_service: None,
      code_info: None,
      
    }
  }
  
  pub fn is_registered(&self) -> Option<bool> { self.is_registered.clone() }
  #[doc(hidden)] pub fn _set_is_registered(&mut self, is_registered: bool) -> &mut Self { self.is_registered = Some(is_registered); self }
  
  pub fn terms_of_service(&self) -> Option<TermsOfService> { self.terms_of_service.clone() }
  #[doc(hidden)] pub fn _set_terms_of_service(&mut self, terms_of_service: TermsOfService) -> &mut Self { self.terms_of_service = Some(terms_of_service); self }
  
  pub fn code_info(&self) -> Option<AuthenticationCodeInfo> { self.code_info.clone() }
  #[doc(hidden)] pub fn _set_code_info(&mut self, code_info: AuthenticationCodeInfo) -> &mut Self { self.code_info = Some(code_info); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



