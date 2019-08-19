
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Quickly returns approximate storage usage statistics. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStorageStatisticsFast {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getStorageStatisticsFast
  
}



impl Object for GetStorageStatisticsFast {}
impl RObject for GetStorageStatisticsFast {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getStorageStatisticsFast" }
  fn td_type(&self) -> RTDType { RTDType::GetStorageStatisticsFast }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetStorageStatisticsFast {}


impl GetStorageStatisticsFast {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getStorageStatisticsFast".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



