
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A member left the chat. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventMemberLeft {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventMemberLeft
  
}



impl Object for ChatEventMemberLeft {}
impl RObject for ChatEventMemberLeft {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberLeft" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventMemberLeft }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventMemberLeft {}


impl ChatEventMemberLeft {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventMemberLeft".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



