
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns bot members of the supergroup or channel. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupergroupMembersFilterBots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // supergroupMembersFilterBots
  
}



impl Object for SupergroupMembersFilterBots {}
impl RObject for SupergroupMembersFilterBots {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "supergroupMembersFilterBots" }
  fn td_type(&self) -> RTDType { RTDType::SupergroupMembersFilterBots }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SupergroupMembersFilter for SupergroupMembersFilterBots {}


impl SupergroupMembersFilterBots {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "supergroupMembersFilterBots".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



