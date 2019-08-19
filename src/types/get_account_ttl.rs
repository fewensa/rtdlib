
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the period of inactivity after which the account of the current user will automatically be deleted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountTtl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getAccountTtl
  
}



impl Object for GetAccountTtl {}
impl RObject for GetAccountTtl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getAccountTtl" }
  fn td_type(&self) -> RTDType { RTDType::GetAccountTtl }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetAccountTtl {}


impl GetAccountTtl {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getAccountTtl".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



