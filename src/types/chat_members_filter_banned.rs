
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns users banned from the chat; can be used only by administrators in a supergroup or in a channel. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMembersFilterBanned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatMembersFilterBanned
  
}



impl Object for ChatMembersFilterBanned {}
impl RObject for ChatMembersFilterBanned {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMembersFilterBanned" }
  fn td_type(&self) -> RTDType { RTDType::ChatMembersFilterBanned }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatMembersFilter for ChatMembersFilterBanned {}


impl ChatMembersFilterBanned {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatMembersFilterBanned".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



