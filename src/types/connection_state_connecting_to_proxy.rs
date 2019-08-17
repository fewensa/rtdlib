
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Currently establishing a connection with a proxy server. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStateConnectingToProxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // connectionStateConnectingToProxy
  
}



impl Object for ConnectionStateConnectingToProxy {}
impl RObject for ConnectionStateConnectingToProxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectionStateConnectingToProxy" }
  fn td_type(&self) -> RTDType { RTDType::ConnectionStateConnectingToProxy }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ConnectionState for ConnectionStateConnectingToProxy {}


impl ConnectionStateConnectingToProxy {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "connectionStateConnectingToProxy".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



