
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Currently waiting for the network to become available. Use SetNetworkType to change the available network type. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStateWaitingForNetwork {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // connectionStateWaitingForNetwork
  
}



impl Object for ConnectionStateWaitingForNetwork {}
impl RObject for ConnectionStateWaitingForNetwork {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectionStateWaitingForNetwork" }
  fn td_type(&self) -> RTDType { RTDType::ConnectionStateWaitingForNetwork }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ConnectionState for ConnectionStateWaitingForNetwork {}


impl ConnectionStateWaitingForNetwork {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "connectionStateWaitingForNetwork".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



