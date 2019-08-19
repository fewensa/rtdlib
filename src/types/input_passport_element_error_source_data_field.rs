
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A data field contains an error. The error is considered resolved when the field's value changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceDataField {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementErrorSourceDataField
  /// Field name.
  field_name: Option<String>,
  /// Current data hash.
  data_hash: Option<String>,
  
}



impl Object for InputPassportElementErrorSourceDataField {}
impl RObject for InputPassportElementErrorSourceDataField {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceDataField" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementErrorSourceDataField }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElementErrorSource for InputPassportElementErrorSourceDataField {}


impl InputPassportElementErrorSourceDataField {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementErrorSourceDataField".to_string(),
      field_name: None,
      data_hash: None,
      
    }
  }
  
  pub fn field_name(&self) -> Option<String> { self.field_name.clone() }
  #[doc(hidden)] pub fn _set_field_name(&mut self, field_name: String) -> &mut Self { self.field_name = Some(field_name); self }
  
  pub fn data_hash(&self) -> Option<String> { self.data_hash.clone() }
  #[doc(hidden)] pub fn _set_data_hash(&mut self, data_hash: String) -> &mut Self { self.data_hash = Some(data_hash); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



