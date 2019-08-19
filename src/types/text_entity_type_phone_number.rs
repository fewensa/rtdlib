
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A phone number. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntityTypePhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntityTypePhoneNumber
  
}



impl Object for TextEntityTypePhoneNumber {}
impl RObject for TextEntityTypePhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypePhoneNumber" }
  fn td_type(&self) -> RTDType { RTDType::TextEntityTypePhoneNumber }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextEntityType for TextEntityTypePhoneNumber {}


impl TextEntityTypePhoneNumber {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntityTypePhoneNumber".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



