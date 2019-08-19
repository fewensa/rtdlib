
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A contact has registered with Telegram. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContactRegistered {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageContactRegistered
  
}



impl Object for MessageContactRegistered {}
impl RObject for MessageContactRegistered {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageContactRegistered" }
  fn td_type(&self) -> RTDType { RTDType::MessageContactRegistered }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageContactRegistered {}


impl MessageContactRegistered {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageContactRegistered".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



