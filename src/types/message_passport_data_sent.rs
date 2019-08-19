
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Telegram Passport data has been sent. 
#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePassportDataSent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messagePassportDataSent
  /// List of Telegram Passport element types sent.
  types: Option<Vec<Box<PassportElementType>>>,
  
}


impl Clone for MessagePassportDataSent {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for MessagePassportDataSent {}
impl RObject for MessagePassportDataSent {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePassportDataSent" }
  fn td_type(&self) -> RTDType { RTDType::MessagePassportDataSent }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessagePassportDataSent {}


impl MessagePassportDataSent {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messagePassportDataSent".to_string(),
      types: None,
      
    }
  }
  
  pub fn types(&self) -> Option<Vec<Box<PassportElementType>>> { self.types.clone() }
  #[doc(hidden)] pub fn _set_types(&mut self, types: Vec<Box<PassportElementType>>) -> &mut Self { self.types = Some(types); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



