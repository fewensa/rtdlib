
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Deletes saved order info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSavedOrderInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deleteSavedOrderInfo
  
}



impl Object for DeleteSavedOrderInfo {}
impl RObject for DeleteSavedOrderInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteSavedOrderInfo" }
  fn td_type(&self) -> RTDType { RTDType::DeleteSavedOrderInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DeleteSavedOrderInfo {}


impl DeleteSavedOrderInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deleteSavedOrderInfo".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



