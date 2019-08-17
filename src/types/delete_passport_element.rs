
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Deletes a Telegram Passport element.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePassportElement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deletePassportElement
  /// Element type.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<PassportElementType>>,
  
}


impl Clone for DeletePassportElement {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for DeletePassportElement {}
impl RObject for DeletePassportElement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deletePassportElement" }
  fn td_type(&self) -> RTDType { RTDType::DeletePassportElement }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DeletePassportElement {}


impl DeletePassportElement {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deletePassportElement".to_string(),
      type_: None,
      
    }
  }
  
  pub fn type_(&self) -> Option<Box<PassportElementType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<PassportElementType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



