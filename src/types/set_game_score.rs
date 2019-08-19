
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Updates the game score of the specified user in the game; for bots only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetGameScore {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setGameScore
  /// The chat to which the message with the game belongs.
  chat_id: Option<i64>,
  /// Identifier of the message.
  message_id: Option<i64>,
  /// True, if the message should be edited.
  edit_message: Option<bool>,
  /// User identifier.
  user_id: Option<i32>,
  /// The new score.
  score: Option<i32>,
  /// Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table.
  force: Option<bool>,
  
}



impl Object for SetGameScore {}
impl RObject for SetGameScore {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setGameScore" }
  fn td_type(&self) -> RTDType { RTDType::SetGameScore }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetGameScore {}


impl SetGameScore {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setGameScore".to_string(),
      chat_id: None,
      message_id: None,
      edit_message: None,
      user_id: None,
      score: None,
      force: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
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



