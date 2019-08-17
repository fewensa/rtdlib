
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the creator and administrators. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMembersFilterAdministrators {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatMembersFilterAdministrators
  
}



impl Object for ChatMembersFilterAdministrators {}
impl RObject for ChatMembersFilterAdministrators {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMembersFilterAdministrators" }
  fn td_type(&self) -> RTDType { RTDType::ChatMembersFilterAdministrators }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatMembersFilter for ChatMembersFilterAdministrators {}


impl ChatMembersFilterAdministrators {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatMembersFilterAdministrators".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



