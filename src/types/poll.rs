
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a poll. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // poll
  /// Unique poll identifier.
  id: Option<String>,
  /// Poll question, 1-255 characters.
  question: Option<String>,
  /// List of poll answer options.
  options: Option<Vec<PollOption>>,
  /// Total number of voters, participating in the poll.
  total_voter_count: Option<i32>,
  /// True, if the poll is closed.
  is_closed: Option<bool>,
  
}



impl Object for Poll {}
impl RObject for Poll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "poll" }
  fn td_type(&self) -> RTDType { RTDType::Poll }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Poll {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "poll".to_string(),
      id: None,
      question: None,
      options: None,
      total_voter_count: None,
      is_closed: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn question(&self) -> Option<String> { self.question.clone() }
  #[doc(hidden)] pub fn _set_question(&mut self, question: String) -> &mut Self { self.question = Some(question); self }
  
  pub fn options(&self) -> Option<Vec<PollOption>> { self.options.clone() }
  #[doc(hidden)] pub fn _set_options(&mut self, options: Vec<PollOption>) -> &mut Self { self.options = Some(options); self }
  
  pub fn total_voter_count(&self) -> Option<i32> { self.total_voter_count.clone() }
  #[doc(hidden)] pub fn _set_total_voter_count(&mut self, total_voter_count: i32) -> &mut Self { self.total_voter_count = Some(total_voter_count); self }
  
  pub fn is_closed(&self) -> Option<bool> { self.is_closed.clone() }
  #[doc(hidden)] pub fn _set_is_closed(&mut self, is_closed: bool) -> &mut Self { self.is_closed = Some(is_closed); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



