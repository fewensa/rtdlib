
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Used to search for supergroup or channel members via a (string) query. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupergroupMembersFilterSearch {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // supergroupMembersFilterSearch
  /// Query to search for.
  query: Option<String>,
  
}



impl Object for SupergroupMembersFilterSearch {}
impl RObject for SupergroupMembersFilterSearch {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "supergroupMembersFilterSearch" }
  fn td_type(&self) -> RTDType { RTDType::SupergroupMembersFilterSearch }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SupergroupMembersFilter for SupergroupMembersFilterSearch {}


impl SupergroupMembersFilterSearch {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "supergroupMembersFilterSearch".to_string(),
      query: None,
      
    }
  }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



