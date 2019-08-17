
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends a Telegram Passport authorization form, effectively sharing data with the service. This method must be called after 
#[derive(Debug, Serialize, Deserialize)]
pub struct SendPassportAuthorizationForm {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sendPassportAuthorizationForm
  /// Authorization form identifier.
  autorization_form_id: Option<i32>,
  /// Types of Telegram Passport elements chosen by user to complete the authorization form.
  types: Option<Vec<Box<PassportElementType>>>,
  
}


impl Clone for SendPassportAuthorizationForm {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SendPassportAuthorizationForm {}
impl RObject for SendPassportAuthorizationForm {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendPassportAuthorizationForm" }
  fn td_type(&self) -> RTDType { RTDType::SendPassportAuthorizationForm }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SendPassportAuthorizationForm {}


impl SendPassportAuthorizationForm {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sendPassportAuthorizationForm".to_string(),
      autorization_form_id: None,
      types: None,
      
    }
  }
  
  pub fn autorization_form_id(&self) -> Option<i32> { self.autorization_form_id.clone() }
  #[doc(hidden)] pub fn _set_autorization_form_id(&mut self, autorization_form_id: i32) -> &mut Self { self.autorization_form_id = Some(autorization_form_id); self }
  
  pub fn types(&self) -> Option<Vec<Box<PassportElementType>>> { self.types.clone() }
  #[doc(hidden)] pub fn _set_types(&mut self, types: Vec<Box<PassportElementType>>) -> &mut Self { self.types = Some(types); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



