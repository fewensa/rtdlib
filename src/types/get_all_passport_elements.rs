
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns all available Telegram Passport elements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAllPassportElements {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getAllPassportElements
  /// Password of the current user.
  password: Option<String>,
  
}



impl Object for GetAllPassportElements {}
impl RObject for GetAllPassportElements {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getAllPassportElements" }
  fn td_type(&self) -> RTDType { RTDType::GetAllPassportElements }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetAllPassportElements {}


impl GetAllPassportElements {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getAllPassportElements".to_string(),
      password: None,
      
    }
  }
  
  pub fn password(&self) -> Option<String> { self.password.clone() }
  #[doc(hidden)] pub fn _set_password(&mut self, password: String) -> &mut Self { self.password = Some(password); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



