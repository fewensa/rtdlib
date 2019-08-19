
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns bot members of the chat. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMembersFilterBots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatMembersFilterBots
  
}



impl Object for ChatMembersFilterBots {}
impl RObject for ChatMembersFilterBots {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMembersFilterBots" }
  fn td_type(&self) -> RTDType { RTDType::ChatMembersFilterBots }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatMembersFilter for ChatMembersFilterBots {}


impl ChatMembersFilterBots {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatMembersFilterBots".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



