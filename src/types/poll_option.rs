
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes one answer option of a poll. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pollOption
  /// Option text, 1-100 characters.
  text: Option<String>,
  /// Number of voters for this option, available only for closed or voted polls.
  voter_count: Option<i32>,
  /// The percentage of votes for this option, 0-100.
  vote_percentage: Option<i32>,
  /// True, if the option was chosen by the user.
  is_chosen: Option<bool>,
  /// True, if the option is being chosen by a pending setPollAnswer request.
  is_being_chosen: Option<bool>,
  
}



impl Object for PollOption {}
impl RObject for PollOption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pollOption" }
  fn td_type(&self) -> RTDType { RTDType::PollOption }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PollOption {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pollOption".to_string(),
      text: None,
      voter_count: None,
      vote_percentage: None,
      is_chosen: None,
      is_being_chosen: None,
      
    }
  }
  
  pub fn text(&self) -> Option<String> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: String) -> &mut Self { self.text = Some(text); self }
  
  pub fn voter_count(&self) -> Option<i32> { self.voter_count.clone() }
  #[doc(hidden)] pub fn _set_voter_count(&mut self, voter_count: i32) -> &mut Self { self.voter_count = Some(voter_count); self }
  
  pub fn vote_percentage(&self) -> Option<i32> { self.vote_percentage.clone() }
  #[doc(hidden)] pub fn _set_vote_percentage(&mut self, vote_percentage: i32) -> &mut Self { self.vote_percentage = Some(vote_percentage); self }
  
  pub fn is_chosen(&self) -> Option<bool> { self.is_chosen.clone() }
  #[doc(hidden)] pub fn _set_is_chosen(&mut self, is_chosen: bool) -> &mut Self { self.is_chosen = Some(is_chosen); self }
  
  pub fn is_being_chosen(&self) -> Option<bool> { self.is_being_chosen.clone() }
  #[doc(hidden)] pub fn _set_is_being_chosen(&mut self, is_being_chosen: bool) -> &mut Self { self.is_being_chosen = Some(is_being_chosen); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



