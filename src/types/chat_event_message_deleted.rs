
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message was deleted. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventMessageDeleted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventMessageDeleted
  /// Deleted message.
  message: Option<Message>,
  
}



impl Object for ChatEventMessageDeleted {}
impl RObject for ChatEventMessageDeleted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMessageDeleted" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventMessageDeleted }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventMessageDeleted {}


impl ChatEventMessageDeleted {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventMessageDeleted".to_string(),
      message: None,
      
    }
  }
  
  pub fn message(&self) -> Option<Message> { self.message.clone() }
  #[doc(hidden)] pub fn _set_message(&mut self, message: Message) -> &mut Self { self.message = Some(message); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



