
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains active notifications that was shown on previous application launches. This update is sent only if a message database is used. In that case it comes once before any 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateActiveNotifications {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateActiveNotifications
  /// Lists of active notification groups.
  groups: Option<Vec<NotificationGroup>>,
  
}



impl Object for UpdateActiveNotifications {}
impl RObject for UpdateActiveNotifications {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateActiveNotifications" }
  fn td_type(&self) -> RTDType { RTDType::UpdateActiveNotifications }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateActiveNotifications {}


impl UpdateActiveNotifications {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateActiveNotifications".to_string(),
      groups: None,
      
    }
  }
  
  pub fn groups(&self) -> Option<Vec<NotificationGroup>> { self.groups.clone() }
  #[doc(hidden)] pub fn _set_groups(&mut self, groups: Vec<NotificationGroup>) -> &mut Self { self.groups = Some(groups); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



