
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains one row of the game high score table. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameHighScore {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // gameHighScore
  /// Position in the high score table.
  position: Option<i32>,
  /// User identifier.
  user_id: Option<i32>,
  /// User score.
  score: Option<i32>,
  
}



impl Object for GameHighScore {}
impl RObject for GameHighScore {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "gameHighScore" }
  fn td_type(&self) -> RTDType { RTDType::GameHighScore }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl GameHighScore {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "gameHighScore".to_string(),
      position: None,
      user_id: None,
      score: None,
      
    }
  }
  
  pub fn position(&self) -> Option<i32> { self.position.clone() }
  #[doc(hidden)] pub fn _set_position(&mut self, position: i32) -> &mut Self { self.position = Some(position); self }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn score(&self) -> Option<i32> { self.score.clone() }
  #[doc(hidden)] pub fn _set_score(&mut self, score: i32) -> &mut Self { self.score = Some(score); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



