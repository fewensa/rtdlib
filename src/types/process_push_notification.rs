
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Handles a push notification. Returns error with code 406 if the push notification is not supported and connection to the server is required to fetch new data. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessPushNotification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // processPushNotification
  /// JSON-encoded push notification payload with all fields sent by the server, and "google.sent_time" and "google.notification.sound" fields added.
  payload: Option<String>,
  
}



impl Object for ProcessPushNotification {}
impl RObject for ProcessPushNotification {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "processPushNotification" }
  fn td_type(&self) -> RTDType { RTDType::ProcessPushNotification }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ProcessPushNotification {}


impl ProcessPushNotification {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "processPushNotification".to_string(),
      payload: None,
      
    }
  }
  
  pub fn payload(&self) -> Option<String> { self.payload.clone() }
  #[doc(hidden)] pub fn _set_payload(&mut self, payload: String) -> &mut Self { self.payload = Some(payload); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



