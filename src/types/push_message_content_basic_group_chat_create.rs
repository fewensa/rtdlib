
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A newly created basic group. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentBasicGroupChatCreate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentBasicGroupChatCreate
  
}



impl Object for PushMessageContentBasicGroupChatCreate {}
impl RObject for PushMessageContentBasicGroupChatCreate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentBasicGroupChatCreate" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentBasicGroupChatCreate }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentBasicGroupChatCreate {}


impl PushMessageContentBasicGroupChatCreate {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentBasicGroupChatCreate".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



