
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the total number of imported contacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportedContactCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getImportedContactCount
  
}



impl Object for GetImportedContactCount {}
impl RObject for GetImportedContactCount {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getImportedContactCount" }
  fn td_type(&self) -> RTDType { RTDType::GetImportedContactCount }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetImportedContactCount {}


impl GetImportedContactCount {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getImportedContactCount".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



