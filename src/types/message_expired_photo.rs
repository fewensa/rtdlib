
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An expired photo message (self-destructed after TTL has elapsed). 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageExpiredPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageExpiredPhoto
  
}



impl Object for MessageExpiredPhoto {}
impl RObject for MessageExpiredPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageExpiredPhoto" }
  fn td_type(&self) -> RTDType { RTDType::MessageExpiredPhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageExpiredPhoto {}


impl MessageExpiredPhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageExpiredPhoto".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



