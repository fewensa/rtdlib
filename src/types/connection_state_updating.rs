
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Downloading data received while the client was offline. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStateUpdating {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // connectionStateUpdating
  
}



impl Object for ConnectionStateUpdating {}
impl RObject for ConnectionStateUpdating {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectionStateUpdating" }
  fn td_type(&self) -> RTDType { RTDType::ConnectionStateUpdating }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ConnectionState for ConnectionStateUpdating {}


impl ConnectionStateUpdating {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "connectionStateUpdating".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



