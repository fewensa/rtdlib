
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The view count of the message has changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessageViews {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateMessageViews
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Message identifier.
  message_id: Option<i64>,
  /// New value of the view count.
  views: Option<i32>,
  
}



impl Object for UpdateMessageViews {}
impl RObject for UpdateMessageViews {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageViews" }
  fn td_type(&self) -> RTDType { RTDType::UpdateMessageViews }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateMessageViews {}


impl UpdateMessageViews {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateMessageViews".to_string(),
      chat_id: None,
      message_id: None,
      views: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn views(&self) -> Option<i32> { self.views.clone() }
  #[doc(hidden)] pub fn _set_views(&mut self, views: i32) -> &mut Self { self.views = Some(views); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



