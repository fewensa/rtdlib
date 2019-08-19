
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new high score was achieved in a game. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentGameScore {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentGameScore
  /// Game title, empty for pinned message.
  title: Option<String>,
  /// New score, 0 for pinned message.
  score: Option<i32>,
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentGameScore {}
impl RObject for PushMessageContentGameScore {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentGameScore" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentGameScore }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentGameScore {}


impl PushMessageContentGameScore {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentGameScore".to_string(),
      title: None,
      score: None,
      is_pinned: None,
      
    }
  }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn score(&self) -> Option<i32> { self.score.clone() }
  #[doc(hidden)] pub fn _set_score(&mut self, score: i32) -> &mut Self { self.score = Some(score); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



