
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with a poll. Polls can't be sent to private or secret chats. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMessagePoll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessagePoll
  /// Poll question, 1-255 characters.
  question: Option<String>,
  /// List of poll answer options, 2-10 strings 1-100 characters each.
  options: Option<Vec<String>>,
  
}



impl Object for InputMessagePoll {}
impl RObject for InputMessagePoll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessagePoll" }
  fn td_type(&self) -> RTDType { RTDType::InputMessagePoll }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessagePoll {}


impl InputMessagePoll {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessagePoll".to_string(),
      question: None,
      options: None,
      
    }
  }
  
  pub fn question(&self) -> Option<String> { self.question.clone() }
  #[doc(hidden)] pub fn _set_question(&mut self, question: String) -> &mut Self { self.question = Some(question); self }
  
  pub fn options(&self) -> Option<Vec<String>> { self.options.clone() }
  #[doc(hidden)] pub fn _set_options(&mut self, options: Vec<String>) -> &mut Self { self.options = Some(options); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



