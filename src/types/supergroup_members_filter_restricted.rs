
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns restricted supergroup members; can be used only by administrators. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupergroupMembersFilterRestricted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // supergroupMembersFilterRestricted
  /// Query to search for.
  query: Option<String>,
  
}



impl Object for SupergroupMembersFilterRestricted {}
impl RObject for SupergroupMembersFilterRestricted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "supergroupMembersFilterRestricted" }
  fn td_type(&self) -> RTDType { RTDType::SupergroupMembersFilterRestricted }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SupergroupMembersFilter for SupergroupMembersFilterRestricted {}


impl SupergroupMembersFilterRestricted {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "supergroupMembersFilterRestricted".to_string(),
      query: None,
      
    }
  }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



