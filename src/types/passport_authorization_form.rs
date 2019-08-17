
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about a Telegram Passport authorization form that was requested. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportAuthorizationForm {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportAuthorizationForm
  /// Unique identifier of the authorization form.
  id: Option<i32>,
  /// Information about the Telegram Passport elements that need to be provided to complete the form.
  required_elements: Option<Vec<PassportRequiredElement>>,
  /// URL for the privacy policy of the service; may be empty.
  privacy_policy_url: Option<String>,
  
}



impl Object for PassportAuthorizationForm {}
impl RObject for PassportAuthorizationForm {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportAuthorizationForm" }
  fn td_type(&self) -> RTDType { RTDType::PassportAuthorizationForm }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PassportAuthorizationForm {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportAuthorizationForm".to_string(),
      id: None,
      required_elements: None,
      privacy_policy_url: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i32> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i32) -> &mut Self { self.id = Some(id); self }
  
  pub fn required_elements(&self) -> Option<Vec<PassportRequiredElement>> { self.required_elements.clone() }
  #[doc(hidden)] pub fn _set_required_elements(&mut self, required_elements: Vec<PassportRequiredElement>) -> &mut Self { self.required_elements = Some(required_elements); self }
  
  pub fn privacy_policy_url(&self) -> Option<String> { self.privacy_policy_url.clone() }
  #[doc(hidden)] pub fn _set_privacy_policy_url(&mut self, privacy_policy_url: String) -> &mut Self { self.privacy_policy_url = Some(privacy_policy_url); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



