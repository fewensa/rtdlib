
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Some data from 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBasicGroupFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateBasicGroupFullInfo
  /// Identifier of a basic group.
  basic_group_id: Option<i32>,
  /// New full information about the group.
  basic_group_full_info: Option<BasicGroupFullInfo>,
  
}



impl Object for UpdateBasicGroupFullInfo {}
impl RObject for UpdateBasicGroupFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateBasicGroupFullInfo" }
  fn td_type(&self) -> RTDType { RTDType::UpdateBasicGroupFullInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateBasicGroupFullInfo {}


impl UpdateBasicGroupFullInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateBasicGroupFullInfo".to_string(),
      basic_group_id: None,
      basic_group_full_info: None,
      
    }
  }
  
  pub fn basic_group_id(&self) -> Option<i32> { self.basic_group_id.clone() }
  #[doc(hidden)] pub fn _set_basic_group_id(&mut self, basic_group_id: i32) -> &mut Self { self.basic_group_id = Some(basic_group_id); self }
  
  pub fn basic_group_full_info(&self) -> Option<BasicGroupFullInfo> { self.basic_group_full_info.clone() }
  #[doc(hidden)] pub fn _set_basic_group_full_info(&mut self, basic_group_full_info: BasicGroupFullInfo) -> &mut Self { self.basic_group_full_info = Some(basic_group_full_info); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



