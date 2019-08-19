
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A full list of available network statistic entries. 
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // networkStatistics
  /// Point in time (Unix timestamp) when the app began collecting statistics.
  since_date: Option<i32>,
  /// Network statistics entries.
  entries: Option<Vec<Box<NetworkStatisticsEntry>>>,
  
}


impl Clone for NetworkStatistics {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for NetworkStatistics {}
impl RObject for NetworkStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkStatistics" }
  fn td_type(&self) -> RTDType { RTDType::NetworkStatistics }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl NetworkStatistics {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "networkStatistics".to_string(),
      since_date: None,
      entries: None,
      
    }
  }
  
  pub fn since_date(&self) -> Option<i32> { self.since_date.clone() }
  #[doc(hidden)] pub fn _set_since_date(&mut self, since_date: i32) -> &mut Self { self.since_date = Some(since_date); self }
  
  pub fn entries(&self) -> Option<Vec<Box<NetworkStatisticsEntry>>> { self.entries.clone() }
  #[doc(hidden)] pub fn _set_entries(&mut self, entries: Vec<Box<NetworkStatisticsEntry>>) -> &mut Self { self.entries = Some(entries); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



