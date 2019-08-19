
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Some data from 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSupergroupFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateSupergroupFullInfo
  /// Identifier of the supergroup or channel.
  supergroup_id: Option<i32>,
  /// New full information about the supergroup.
  supergroup_full_info: Option<SupergroupFullInfo>,
  
}



impl Object for UpdateSupergroupFullInfo {}
impl RObject for UpdateSupergroupFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateSupergroupFullInfo" }
  fn td_type(&self) -> RTDType { RTDType::UpdateSupergroupFullInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateSupergroupFullInfo {}


impl UpdateSupergroupFullInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateSupergroupFullInfo".to_string(),
      supergroup_id: None,
      supergroup_full_info: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn supergroup_full_info(&self) -> Option<SupergroupFullInfo> { self.supergroup_full_info.clone() }
  #[doc(hidden)] pub fn _set_supergroup_full_info(&mut self, supergroup_full_info: SupergroupFullInfo) -> &mut Self { self.supergroup_full_info = Some(supergroup_full_info); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



