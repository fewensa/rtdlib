
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes, whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHavePendingNotifications {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateHavePendingNotifications
  /// True, if there are some delayed notification updates, which will be sent soon.
  have_delayed_notifications: Option<bool>,
  /// True, if there can be some yet unreceived notifications, which are being fetched from the server.
  have_unreceived_notifications: Option<bool>,
  
}



impl Object for UpdateHavePendingNotifications {}
impl RObject for UpdateHavePendingNotifications {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateHavePendingNotifications" }
  fn td_type(&self) -> RTDType { RTDType::UpdateHavePendingNotifications }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateHavePendingNotifications {}


impl UpdateHavePendingNotifications {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateHavePendingNotifications".to_string(),
      have_delayed_notifications: None,
      have_unreceived_notifications: None,
      
    }
  }
  
  pub fn have_delayed_notifications(&self) -> Option<bool> { self.have_delayed_notifications.clone() }
  #[doc(hidden)] pub fn _set_have_delayed_notifications(&mut self, have_delayed_notifications: bool) -> &mut Self { self.have_delayed_notifications = Some(have_delayed_notifications); self }
  
  pub fn have_unreceived_notifications(&self) -> Option<bool> { self.have_unreceived_notifications.clone() }
  #[doc(hidden)] pub fn _set_have_unreceived_notifications(&mut self, have_unreceived_notifications: bool) -> &mut Self { self.have_unreceived_notifications = Some(have_unreceived_notifications); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



