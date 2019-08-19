
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns users under certain restrictions in the chat; can be used only by administrators in a supergroup. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMembersFilterRestricted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatMembersFilterRestricted
  
}



impl Object for ChatMembersFilterRestricted {}
impl RObject for ChatMembersFilterRestricted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMembersFilterRestricted" }
  fn td_type(&self) -> RTDType { RTDType::ChatMembersFilterRestricted }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatMembersFilter for ChatMembersFilterRestricted {}


impl ChatMembersFilterRestricted {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatMembersFilterRestricted".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



