
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with a game. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentGame
  /// Game title, empty for pinned game message.
  title: Option<String>,
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentGame {}
impl RObject for PushMessageContentGame {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentGame" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentGame }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentGame {}


impl PushMessageContentGame {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentGame".to_string(),
      title: None,
      is_pinned: None,
      
    }
  }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



