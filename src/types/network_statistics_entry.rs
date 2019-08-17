
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Contains statistics about network usage. 
#[typetag::serde(tag = "@struct")]
pub trait NetworkStatisticsEntry: Object + RObject + Debug {}






impl NetworkStatisticsEntry {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<NetworkStatisticsEntry> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDNetworkStatisticsEntryType {
  NetworkStatisticsEntryCall,
  NetworkStatisticsEntryFile,
  
}
impl RTDNetworkStatisticsEntryType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDNetworkStatisticsEntryType)(text.as_ref()) }
}



