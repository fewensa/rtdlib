
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatDefaultDisableNotification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateChatDefaultDisableNotification
  /// Chat identifier.
  chat_id: Option<i64>,
  /// The new default_disable_notification value.
  default_disable_notification: Option<bool>,
  
}



impl Object for UpdateChatDefaultDisableNotification {}
impl RObject for UpdateChatDefaultDisableNotification {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatDefaultDisableNotification" }
  fn td_type(&self) -> RTDType { RTDType::UpdateChatDefaultDisableNotification }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateChatDefaultDisableNotification {}


impl UpdateChatDefaultDisableNotification {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateChatDefaultDisableNotification".to_string(),
      chat_id: None,
      default_disable_notification: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn default_disable_notification(&self) -> Option<bool> { self.default_disable_notification.clone() }
  #[doc(hidden)] pub fn _set_default_disable_notification(&mut self, default_disable_notification: bool) -> &mut Self { self.default_disable_notification = Some(default_disable_notification); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



