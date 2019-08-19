
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns already available Telegram Passport elements suitable for completing a Telegram Passport authorization form. Result can be received only once for each authorization form.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPassportAuthorizationFormAvailableElements {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getPassportAuthorizationFormAvailableElements
  /// Authorization form identifier.
  autorization_form_id: Option<i32>,
  /// Password of the current user.
  password: Option<String>,
  
}



impl Object for GetPassportAuthorizationFormAvailableElements {}
impl RObject for GetPassportAuthorizationFormAvailableElements {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPassportAuthorizationFormAvailableElements" }
  fn td_type(&self) -> RTDType { RTDType::GetPassportAuthorizationFormAvailableElements }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetPassportAuthorizationFormAvailableElements {}


impl GetPassportAuthorizationFormAvailableElements {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getPassportAuthorizationFormAvailableElements".to_string(),
      autorization_form_id: None,
      password: None,
      
    }
  }
  
  pub fn autorization_form_id(&self) -> Option<i32> { self.autorization_form_id.clone() }
  #[doc(hidden)] pub fn _set_autorization_form_id(&mut self, autorization_form_id: i32) -> &mut Self { self.autorization_form_id = Some(autorization_form_id); self }
  
  pub fn password(&self) -> Option<String> { self.password.clone() }
  #[doc(hidden)] pub fn _set_password(&mut self, password: String) -> &mut Self { self.password = Some(password); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



