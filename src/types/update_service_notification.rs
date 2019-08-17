
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Service notification from the server. Upon receiving this the client must show a popup with the content of the notification. 
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateServiceNotification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateServiceNotification
  /// Notification type. If type begins with "AUTH_KEY_DROP_", then two buttons "Cancel" and "Log out" should be shown under notification; if user presses the second, all local data should be destroyed using Destroy method.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<String>,
  /// Notification content.
  content: Option<Box<MessageContent>>,
  
}


impl Clone for UpdateServiceNotification {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UpdateServiceNotification {}
impl RObject for UpdateServiceNotification {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateServiceNotification" }
  fn td_type(&self) -> RTDType { RTDType::UpdateServiceNotification }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateServiceNotification {}


impl UpdateServiceNotification {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateServiceNotification".to_string(),
      type_: None,
      content: None,
      
    }
  }
  
  pub fn type_(&self) -> Option<String> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: String) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn content(&self) -> Option<Box<MessageContent>> { self.content.clone() }
  #[doc(hidden)] pub fn _set_content(&mut self, content: Box<MessageContent>) -> &mut Self { self.content = Some(content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



