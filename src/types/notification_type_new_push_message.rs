
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// New message was received through a push notification. 
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationTypeNewPushMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // notificationTypeNewPushMessage
  /// The message identifier. The message will not be available in the chat history, but the ID can be used in viewMessages and as reply_to_message_id.
  message_id: Option<i64>,
  /// Sender of the message. Corresponding user may be inaccessible.
  sender_user_id: Option<i32>,
  /// Push message content.
  content: Option<Box<PushMessageContent>>,
  
}


impl Clone for NotificationTypeNewPushMessage {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for NotificationTypeNewPushMessage {}
impl RObject for NotificationTypeNewPushMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationTypeNewPushMessage" }
  fn td_type(&self) -> RTDType { RTDType::NotificationTypeNewPushMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NotificationType for NotificationTypeNewPushMessage {}


impl NotificationTypeNewPushMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "notificationTypeNewPushMessage".to_string(),
      message_id: None,
      sender_user_id: None,
      content: None,
      
    }
  }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn sender_user_id(&self) -> Option<i32> { self.sender_user_id.clone() }
  #[doc(hidden)] pub fn _set_sender_user_id(&mut self, sender_user_id: i32) -> &mut Self { self.sender_user_id = Some(sender_user_id); self }
  
  pub fn content(&self) -> Option<Box<PushMessageContent>> { self.content.clone() }
  #[doc(hidden)] pub fn _set_content(&mut self, content: Box<PushMessageContent>) -> &mut Self { self.content = Some(content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



