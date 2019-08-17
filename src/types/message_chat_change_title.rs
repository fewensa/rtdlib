
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An updated chat title. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageChatChangeTitle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageChatChangeTitle
  /// New chat title.
  title: Option<String>,
  
}



impl Object for MessageChatChangeTitle {}
impl RObject for MessageChatChangeTitle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatChangeTitle" }
  fn td_type(&self) -> RTDType { RTDType::MessageChatChangeTitle }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageChatChangeTitle {}


impl MessageChatChangeTitle {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageChatChangeTitle".to_string(),
      title: None,
      
    }
  }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



