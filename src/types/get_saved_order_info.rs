
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns saved order info, if any.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSavedOrderInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getSavedOrderInfo
  
}



impl Object for GetSavedOrderInfo {}
impl RObject for GetSavedOrderInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getSavedOrderInfo" }
  fn td_type(&self) -> RTDType { RTDType::GetSavedOrderInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetSavedOrderInfo {}


impl GetSavedOrderInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getSavedOrderInfo".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



