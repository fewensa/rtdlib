
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The list of attached files contains an error. The error will be considered resolved when the list of files changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorSourceFiles {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementErrorSourceFiles
  
}



impl Object for PassportElementErrorSourceFiles {}
impl RObject for PassportElementErrorSourceFiles {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceFiles" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementErrorSourceFiles }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementErrorSource for PassportElementErrorSourceFiles {}


impl PassportElementErrorSourceFiles {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementErrorSourceFiles".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



