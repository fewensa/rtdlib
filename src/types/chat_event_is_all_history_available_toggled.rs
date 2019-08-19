
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The is_all_history_available setting of a supergroup was toggled. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventIsAllHistoryAvailableToggled {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventIsAllHistoryAvailableToggled
  /// New value of is_all_history_available.
  is_all_history_available: Option<bool>,
  
}



impl Object for ChatEventIsAllHistoryAvailableToggled {}
impl RObject for ChatEventIsAllHistoryAvailableToggled {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventIsAllHistoryAvailableToggled" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventIsAllHistoryAvailableToggled }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventIsAllHistoryAvailableToggled {}


impl ChatEventIsAllHistoryAvailableToggled {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventIsAllHistoryAvailableToggled".to_string(),
      is_all_history_available: None,
      
    }
  }
  
  pub fn is_all_history_available(&self) -> Option<bool> { self.is_all_history_available.clone() }
  #[doc(hidden)] pub fn _set_is_all_history_available(&mut self, is_all_history_available: bool) -> &mut Self { self.is_all_history_available = Some(is_all_history_available); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



