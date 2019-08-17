
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a list of game high scores. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameHighScores {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // gameHighScores
  /// A list of game high scores.
  scores: Option<Vec<GameHighScore>>,
  
}



impl Object for GameHighScores {}
impl RObject for GameHighScores {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "gameHighScores" }
  fn td_type(&self) -> RTDType { RTDType::GameHighScores }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl GameHighScores {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "gameHighScores".to_string(),
      scores: None,
      
    }
  }
  
  pub fn scores(&self) -> Option<Vec<GameHighScore>> { self.scores.clone() }
  #[doc(hidden)] pub fn _set_scores(&mut self, scores: Vec<GameHighScore>) -> &mut Self { self.scores = Some(scores); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



