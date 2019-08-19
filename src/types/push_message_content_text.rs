
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A text message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentText
  /// Message text.
  text: Option<String>,
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentText {}
impl RObject for PushMessageContentText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentText" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentText }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentText {}


impl PushMessageContentText {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentText".to_string(),
      text: None,
      is_pinned: None,
      
    }
  }
  
  pub fn text(&self) -> Option<String> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: String) -> &mut Self { self.text = Some(text); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



