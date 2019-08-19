
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Currently establishing a connection to the Telegram servers. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStateConnecting {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // connectionStateConnecting
  
}



impl Object for ConnectionStateConnecting {}
impl RObject for ConnectionStateConnecting {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectionStateConnecting" }
  fn td_type(&self) -> RTDType { RTDType::ConnectionStateConnecting }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ConnectionState for ConnectionStateConnecting {}


impl ConnectionStateConnecting {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "connectionStateConnecting".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



