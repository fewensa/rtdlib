
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Updates the game score of the specified user in a game; for bots only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetInlineGameScore {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setInlineGameScore
  /// Inline message identifier.
  inline_message_id: Option<String>,
  /// True, if the message should be edited.
  edit_message: Option<bool>,
  /// User identifier.
  user_id: Option<i32>,
  /// The new score.
  score: Option<i32>,
  /// Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table.
  force: Option<bool>,
  
}



impl Object for SetInlineGameScore {}
impl RObject for SetInlineGameScore {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setInlineGameScore" }
  fn td_type(&self) -> RTDType { RTDType::SetInlineGameScore }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetInlineGameScore {}


impl SetInlineGameScore {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setInlineGameScore".to_string(),
      inline_message_id: None,
      edit_message: None,
      user_id: None,
      score: None,
      force: None,
      
    }
  }
  
  pub fn inline_message_id(&self) -> Option<String> { self.inline_message_id.clone() }
  #[doc(hidden)] pub fn _set_inline_message_id(&mut self, inline_message_id: String) -> &mut Self { self.inline_message_id = Some(inline_message_id); self }
  
  pub fn edit_message(&self) -> Option<bool> { self.edit_message.clone() }
  #[doc(hidden)] pub fn _set_edit_message(&mut self, edit_message: bool) -> &mut Self { self.edit_message = Some(edit_message); self }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn score(&self) -> Option<i32> { self.score.clone() }
  #[doc(hidden)] pub fn _set_score(&mut self, score: i32) -> &mut Self { self.score = Some(score); self }
  
  pub fn force(&self) -> Option<bool> { self.force.clone() }
  #[doc(hidden)] pub fn _set_force(&mut self, force: bool) -> &mut Self { self.force = Some(force); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



