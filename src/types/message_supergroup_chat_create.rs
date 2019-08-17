
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A newly created supergroup or channel. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSupergroupChatCreate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageSupergroupChatCreate
  /// Title of the supergroup or channel.
  title: Option<String>,
  
}



impl Object for MessageSupergroupChatCreate {}
impl RObject for MessageSupergroupChatCreate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageSupergroupChatCreate" }
  fn td_type(&self) -> RTDType { RTDType::MessageSupergroupChatCreate }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageSupergroupChatCreate {}


impl MessageSupergroupChatCreate {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageSupergroupChatCreate".to_string(),
      title: None,
      
    }
  }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



