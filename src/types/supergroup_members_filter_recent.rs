
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns recently active users in reverse chronological order. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupergroupMembersFilterRecent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // supergroupMembersFilterRecent
  
}



impl Object for SupergroupMembersFilterRecent {}
impl RObject for SupergroupMembersFilterRecent {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "supergroupMembersFilterRecent" }
  fn td_type(&self) -> RTDType { RTDType::SupergroupMembersFilterRecent }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SupergroupMembersFilter for SupergroupMembersFilterRecent {}


impl SupergroupMembersFilterRecent {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "supergroupMembersFilterRecent".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



