
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Message content that is not supported by the client. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageUnsupported {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageUnsupported
  
}



impl Object for MessageUnsupported {}
impl RObject for MessageUnsupported {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageUnsupported" }
  fn td_type(&self) -> RTDType { RTDType::MessageUnsupported }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageUnsupported {}


impl MessageUnsupported {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageUnsupported".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



