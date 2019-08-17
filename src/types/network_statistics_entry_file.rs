
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about the total amount of data that was used to send and receive files. 
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStatisticsEntryFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // networkStatisticsEntryFile
  /// Type of the file the data is part of.
  file_type: Option<Box<FileType>>,
  /// Type of the network the data was sent through. Call setNetworkType to maintain the actual network type.
  network_type: Option<Box<NetworkType>>,
  /// Total number of bytes sent.
  sent_bytes: Option<i64>,
  /// Total number of bytes received.
  received_bytes: Option<i64>,
  
}


impl Clone for NetworkStatisticsEntryFile {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for NetworkStatisticsEntryFile {}
impl RObject for NetworkStatisticsEntryFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkStatisticsEntryFile" }
  fn td_type(&self) -> RTDType { RTDType::NetworkStatisticsEntryFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NetworkStatisticsEntry for NetworkStatisticsEntryFile {}


impl NetworkStatisticsEntryFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "networkStatisticsEntryFile".to_string(),
      file_type: None,
      network_type: None,
      sent_bytes: None,
      received_bytes: None,
      
    }
  }
  
  pub fn file_type(&self) -> Option<Box<FileType>> { self.file_type.clone() }
  #[doc(hidden)] pub fn _set_file_type(&mut self, file_type: Box<FileType>) -> &mut Self { self.file_type = Some(file_type); self }
  
  pub fn network_type(&self) -> Option<Box<NetworkType>> { self.network_type.clone() }
  #[doc(hidden)] pub fn _set_network_type(&mut self, network_type: Box<NetworkType>) -> &mut Self { self.network_type = Some(network_type); self }
  
  pub fn sent_bytes(&self) -> Option<i64> { self.sent_bytes.clone() }
  #[doc(hidden)] pub fn _set_sent_bytes(&mut self, sent_bytes: i64) -> &mut Self { self.sent_bytes = Some(sent_bytes); self }
  
  pub fn received_bytes(&self) -> Option<i64> { self.received_bytes.clone() }
  #[doc(hidden)] pub fn _set_received_bytes(&mut self, received_bytes: i64) -> &mut Self { self.received_bytes = Some(received_bytes); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



