
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Toggles the "All members are admins" setting in basic groups; requires creator privileges in the group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToggleBasicGroupAdministrators {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // toggleBasicGroupAdministrators
  /// Identifier of the basic group.
  basic_group_id: Option<i32>,
  /// New value of everyone_is_administrator.
  everyone_is_administrator: Option<bool>,
  
}



impl Object for ToggleBasicGroupAdministrators {}
impl RObject for ToggleBasicGroupAdministrators {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "toggleBasicGroupAdministrators" }
  fn td_type(&self) -> RTDType { RTDType::ToggleBasicGroupAdministrators }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ToggleBasicGroupAdministrators {}


impl ToggleBasicGroupAdministrators {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "toggleBasicGroupAdministrators".to_string(),
      basic_group_id: None,
      everyone_is_administrator: None,
      
    }
  }
  
  pub fn basic_group_id(&self) -> Option<i32> { self.basic_group_id.clone() }
  #[doc(hidden)] pub fn _set_basic_group_id(&mut self, basic_group_id: i32) -> &mut Self { self.basic_group_id = Some(basic_group_id); self }
  
  pub fn everyone_is_administrator(&self) -> Option<bool> { self.everyone_is_administrator.clone() }
  #[doc(hidden)] pub fn _set_everyone_is_administrator(&mut self, everyone_is_administrator: bool) -> &mut Self { self.everyone_is_administrator = Some(everyone_is_administrator); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



