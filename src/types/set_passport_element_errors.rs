
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Informs the user that some of the elements in their Telegram Passport contain errors; for bots only. The user will not be able to resend the elements, until the errors are fixed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPassportElementErrors {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setPassportElementErrors
  /// User identifier.
  user_id: Option<i32>,
  /// The errors.
  errors: Option<Vec<InputPassportElementError>>,
  
}



impl Object for SetPassportElementErrors {}
impl RObject for SetPassportElementErrors {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setPassportElementErrors" }
  fn td_type(&self) -> RTDType { RTDType::SetPassportElementErrors }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetPassportElementErrors {}


impl SetPassportElementErrors {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setPassportElementErrors".to_string(),
      user_id: None,
      errors: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn errors(&self) -> Option<Vec<InputPassportElementError>> { self.errors.clone() }
  #[doc(hidden)] pub fn _set_errors(&mut self, errors: Vec<InputPassportElementError>) -> &mut Self { self.errors = Some(errors); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



