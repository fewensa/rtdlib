
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns database statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDatabaseStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getDatabaseStatistics
  
}



impl Object for GetDatabaseStatistics {}
impl RObject for GetDatabaseStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getDatabaseStatistics" }
  fn td_type(&self) -> RTDType { RTDType::GetDatabaseStatistics }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetDatabaseStatistics {}


impl GetDatabaseStatistics {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getDatabaseStatistics".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



