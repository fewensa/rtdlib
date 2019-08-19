
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Reports some messages from a user in a supergroup as spam; requires administrator rights in the supergroup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSupergroupSpam {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // reportSupergroupSpam
  /// Supergroup identifier.
  supergroup_id: Option<i32>,
  /// User identifier.
  user_id: Option<i32>,
  /// Identifiers of messages sent in the supergroup by the user. This list must be non-empty.
  message_ids: Option<Vec<i64>>,
  
}



impl Object for ReportSupergroupSpam {}
impl RObject for ReportSupergroupSpam {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "reportSupergroupSpam" }
  fn td_type(&self) -> RTDType { RTDType::ReportSupergroupSpam }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ReportSupergroupSpam {}


impl ReportSupergroupSpam {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "reportSupergroupSpam".to_string(),
      supergroup_id: None,
      user_id: None,
      message_ids: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn message_ids(&self) -> Option<Vec<i64>> { self.message_ids.clone() }
  #[doc(hidden)] pub fn _set_message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self { self.message_ids = Some(message_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



