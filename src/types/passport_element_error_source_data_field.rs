
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// One of the data fields contains an error. The error will be considered resolved when the value of the field changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorSourceDataField {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementErrorSourceDataField
  /// Field name.
  field_name: Option<String>,
  
}



impl Object for PassportElementErrorSourceDataField {}
impl RObject for PassportElementErrorSourceDataField {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceDataField" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementErrorSourceDataField }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementErrorSource for PassportElementErrorSourceDataField {}


impl PassportElementErrorSourceDataField {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementErrorSourceDataField".to_string(),
      field_name: None,
      
    }
  }
  
  pub fn field_name(&self) -> Option<String> { self.field_name.clone() }
  #[doc(hidden)] pub fn _set_field_name(&mut self, field_name: String) -> &mut Self { self.field_name = Some(field_name); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



