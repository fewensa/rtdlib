
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns full information about a supergroup or channel by its identifier, cached for up to 1 minute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSupergroupFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getSupergroupFullInfo
  /// Supergroup or channel identifier.
  supergroup_id: Option<i32>,
  
}



impl Object for GetSupergroupFullInfo {}
impl RObject for GetSupergroupFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getSupergroupFullInfo" }
  fn td_type(&self) -> RTDType { RTDType::GetSupergroupFullInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetSupergroupFullInfo {}


impl GetSupergroupFullInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getSupergroupFullInfo".to_string(),
      supergroup_id: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



