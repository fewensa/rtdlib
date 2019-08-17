
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns one of the available Telegram Passport elements.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetPassportElement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getPassportElement
  /// Telegram Passport element type.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<PassportElementType>>,
  /// Password of the current user.
  password: Option<String>,
  
}


impl Clone for GetPassportElement {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for GetPassportElement {}
impl RObject for GetPassportElement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPassportElement" }
  fn td_type(&self) -> RTDType { RTDType::GetPassportElement }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetPassportElement {}


impl GetPassportElement {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getPassportElement".to_string(),
      type_: None,
      password: None,
      
    }
  }
  
  pub fn type_(&self) -> Option<Box<PassportElementType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<PassportElementType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn password(&self) -> Option<String> { self.password.clone() }
  #[doc(hidden)] pub fn _set_password(&mut self, password: String) -> &mut Self { self.password = Some(password); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



