
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An expired video message (self-destructed after TTL has elapsed). 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageExpiredVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageExpiredVideo
  
}



impl Object for MessageExpiredVideo {}
impl RObject for MessageExpiredVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageExpiredVideo" }
  fn td_type(&self) -> RTDType { RTDType::MessageExpiredVideo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageExpiredVideo {}


impl MessageExpiredVideo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageExpiredVideo".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



