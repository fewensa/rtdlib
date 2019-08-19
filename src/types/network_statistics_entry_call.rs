
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about the total amount of data that was used for calls. 
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStatisticsEntryCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // networkStatisticsEntryCall
  /// Type of the network the data was sent through. Call setNetworkType to maintain the actual network type.
  network_type: Option<Box<NetworkType>>,
  /// Total number of bytes sent.
  sent_bytes: Option<i64>,
  /// Total number of bytes received.
  received_bytes: Option<i64>,
  /// Total call duration, in seconds.
  duration: Option<f64>,
  
}


impl Clone for NetworkStatisticsEntryCall {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for NetworkStatisticsEntryCall {}
impl RObject for NetworkStatisticsEntryCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkStatisticsEntryCall" }
  fn td_type(&self) -> RTDType { RTDType::NetworkStatisticsEntryCall }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NetworkStatisticsEntry for NetworkStatisticsEntryCall {}


impl NetworkStatisticsEntryCall {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "networkStatisticsEntryCall".to_string(),
      network_type: None,
      sent_bytes: None,
      received_bytes: None,
      duration: None,
      
    }
  }
  
  pub fn network_type(&self) -> Option<Box<NetworkType>> { self.network_type.clone() }
  #[doc(hidden)] pub fn _set_network_type(&mut self, network_type: Box<NetworkType>) -> &mut Self { self.network_type = Some(network_type); self }
  
  pub fn sent_bytes(&self) -> Option<i64> { self.sent_bytes.clone() }
  #[doc(hidden)] pub fn _set_sent_bytes(&mut self, sent_bytes: i64) -> &mut Self { self.sent_bytes = Some(sent_bytes); self }
  
  pub fn received_bytes(&self) -> Option<i64> { self.received_bytes.clone() }
  #[doc(hidden)] pub fn _set_received_bytes(&mut self, received_bytes: i64) -> &mut Self { self.received_bytes = Some(received_bytes); self }
  
  pub fn duration(&self) -> Option<f64> { self.duration.clone() }
  #[doc(hidden)] pub fn _set_duration(&mut self, duration: f64) -> &mut Self { self.duration = Some(duration); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



