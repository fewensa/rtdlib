
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the creator and administrators. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupergroupMembersFilterAdministrators {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // supergroupMembersFilterAdministrators
  
}



impl Object for SupergroupMembersFilterAdministrators {}
impl RObject for SupergroupMembersFilterAdministrators {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "supergroupMembersFilterAdministrators" }
  fn td_type(&self) -> RTDType { RTDType::SupergroupMembersFilterAdministrators }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SupergroupMembersFilter for SupergroupMembersFilterAdministrators {}


impl SupergroupMembersFilterAdministrators {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "supergroupMembersFilterAdministrators".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



