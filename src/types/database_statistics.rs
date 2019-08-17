
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains database statistics. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // databaseStatistics
  /// Database statistics in an unspecified human-readable format.
  statistics: Option<String>,
  
}



impl Object for DatabaseStatistics {}
impl RObject for DatabaseStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "databaseStatistics" }
  fn td_type(&self) -> RTDType { RTDType::DatabaseStatistics }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl DatabaseStatistics {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "databaseStatistics".to_string(),
      statistics: None,
      
    }
  }
  
  pub fn statistics(&self) -> Option<String> { self.statistics.clone() }
  #[doc(hidden)] pub fn _set_statistics(&mut self, statistics: String) -> &mut Self { self.statistics = Some(statistics); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



