
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Toggles whether the message history of a supergroup is available to new members; requires appropriate administrator rights in the supergroup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToggleSupergroupIsAllHistoryAvailable {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // toggleSupergroupIsAllHistoryAvailable
  /// The identifier of the supergroup.
  supergroup_id: Option<i32>,
  /// The new value of is_all_history_available.
  is_all_history_available: Option<bool>,
  
}



impl Object for ToggleSupergroupIsAllHistoryAvailable {}
impl RObject for ToggleSupergroupIsAllHistoryAvailable {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "toggleSupergroupIsAllHistoryAvailable" }
  fn td_type(&self) -> RTDType { RTDType::ToggleSupergroupIsAllHistoryAvailable }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ToggleSupergroupIsAllHistoryAvailable {}


impl ToggleSupergroupIsAllHistoryAvailable {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "toggleSupergroupIsAllHistoryAvailable".to_string(),
      supergroup_id: None,
      is_all_history_available: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn is_all_history_available(&self) -> Option<bool> { self.is_all_history_available.clone() }
  #[doc(hidden)] pub fn _set_is_all_history_available(&mut self, is_all_history_available: bool) -> &mut Self { self.is_all_history_available = Some(is_all_history_available); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



