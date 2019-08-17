
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A contact has registered with Telegram. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentContactRegistered {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentContactRegistered
  
}



impl Object for PushMessageContentContactRegistered {}
impl RObject for PushMessageContentContactRegistered {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentContactRegistered" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentContactRegistered }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentContactRegistered {}


impl PushMessageContentContactRegistered {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentContactRegistered".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



