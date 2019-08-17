
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Clears all imported contacts, contact list remains unchanged.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearImportedContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // clearImportedContacts
  
}



impl Object for ClearImportedContacts {}
impl RObject for ClearImportedContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "clearImportedContacts" }
  fn td_type(&self) -> RTDType { RTDType::ClearImportedContacts }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ClearImportedContacts {}


impl ClearImportedContacts {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "clearImportedContacts".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



