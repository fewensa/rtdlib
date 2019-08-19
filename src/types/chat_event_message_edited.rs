
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message was edited. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventMessageEdited {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventMessageEdited
  /// The original message before the edit.
  old_message: Option<Message>,
  /// The message after it was edited.
  new_message: Option<Message>,
  
}



impl Object for ChatEventMessageEdited {}
impl RObject for ChatEventMessageEdited {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMessageEdited" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventMessageEdited }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventMessageEdited {}


impl ChatEventMessageEdited {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventMessageEdited".to_string(),
      old_message: None,
      new_message: None,
      
    }
  }
  
  pub fn old_message(&self) -> Option<Message> { self.old_message.clone() }
  #[doc(hidden)] pub fn _set_old_message(&mut self, old_message: Message) -> &mut Self { self.old_message = Some(old_message); self }
  
  pub fn new_message(&self) -> Option<Message> { self.new_message.clone() }
  #[doc(hidden)] pub fn _set_new_message(&mut self, new_message: Message) -> &mut Self { self.new_message = Some(new_message); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



