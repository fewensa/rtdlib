
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A chat title was edited. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentChatChangeTitle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentChatChangeTitle
  /// New chat title.
  title: Option<String>,
  
}



impl Object for PushMessageContentChatChangeTitle {}
impl RObject for PushMessageContentChatChangeTitle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentChatChangeTitle" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentChatChangeTitle }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentChatChangeTitle {}


impl PushMessageContentChatChangeTitle {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentChatChangeTitle".to_string(),
      title: None,
      
    }
  }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



