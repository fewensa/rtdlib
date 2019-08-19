
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// There is a working connection to the Telegram servers. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStateReady {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // connectionStateReady
  
}



impl Object for ConnectionStateReady {}
impl RObject for ConnectionStateReady {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectionStateReady" }
  fn td_type(&self) -> RTDType { RTDType::ConnectionStateReady }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ConnectionState for ConnectionStateReady {}


impl ConnectionStateReady {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "connectionStateReady".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



