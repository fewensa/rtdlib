
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds the specified data to data usage statistics. Can be called before authorization.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddNetworkStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // addNetworkStatistics
  /// The network statistics entry with the data to be added to statistics.
  entry: Option<Box<NetworkStatisticsEntry>>,
  
}


impl Clone for AddNetworkStatistics {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for AddNetworkStatistics {}
impl RObject for AddNetworkStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addNetworkStatistics" }
  fn td_type(&self) -> RTDType { RTDType::AddNetworkStatistics }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AddNetworkStatistics {}


impl AddNetworkStatistics {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "addNetworkStatistics".to_string(),
      entry: None,
      
    }
  }
  
  pub fn entry(&self) -> Option<Box<NetworkStatisticsEntry>> { self.entry.clone() }
  #[doc(hidden)] pub fn _set_entry(&mut self, entry: Box<NetworkStatisticsEntry>) -> &mut Self { self.entry = Some(entry); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



