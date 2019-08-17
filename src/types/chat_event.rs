
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a chat event. 
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatEvent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEvent
  /// Chat event identifier.
  id: Option<i64>,
  /// Point in time (Unix timestamp) when the event happened.
  date: Option<i32>,
  /// Identifier of the user who performed the action that triggered the event.
  user_id: Option<i32>,
  /// Action performed by the user.
  action: Option<Box<ChatEventAction>>,
  
}


impl Clone for ChatEvent {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for ChatEvent {}
impl RObject for ChatEvent {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEvent" }
  fn td_type(&self) -> RTDType { RTDType::ChatEvent }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ChatEvent {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEvent".to_string(),
      id: None,
      date: None,
      user_id: None,
      action: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn date(&self) -> Option<i32> { self.date.clone() }
  #[doc(hidden)] pub fn _set_date(&mut self, date: i32) -> &mut Self { self.date = Some(date); self }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn action(&self) -> Option<Box<ChatEventAction>> { self.action.clone() }
  #[doc(hidden)] pub fn _set_action(&mut self, action: Box<ChatEventAction>) -> &mut Self { self.action = Some(action); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



