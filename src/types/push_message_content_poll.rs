
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with a poll. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentPoll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentPoll
  /// Poll question.
  question: Option<String>,
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentPoll {}
impl RObject for PushMessageContentPoll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentPoll" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentPoll }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentPoll {}


impl PushMessageContentPoll {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentPoll".to_string(),
      question: None,
      is_pinned: None,
      
    }
  }
  
  pub fn question(&self) -> Option<String> { self.question.clone() }
  #[doc(hidden)] pub fn _set_question(&mut self, question: String) -> &mut Self { self.question = Some(question); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



