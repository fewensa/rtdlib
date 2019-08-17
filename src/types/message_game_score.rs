
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new high score was achieved in a game. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageGameScore {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageGameScore
  /// Identifier of the message with the game, can be an identifier of a deleted message.
  game_message_id: Option<i64>,
  /// Identifier of the game; may be different from the games presented in the message with the game.
  game_id: Option<i64>,
  /// New score.
  score: Option<i32>,
  
}



impl Object for MessageGameScore {}
impl RObject for MessageGameScore {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageGameScore" }
  fn td_type(&self) -> RTDType { RTDType::MessageGameScore }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageGameScore {}


impl MessageGameScore {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageGameScore".to_string(),
      game_message_id: None,
      game_id: None,
      score: None,
      
    }
  }
  
  pub fn game_message_id(&self) -> Option<i64> { self.game_message_id.clone() }
  #[doc(hidden)] pub fn _set_game_message_id(&mut self, game_message_id: i64) -> &mut Self { self.game_message_id = Some(game_message_id); self }
  
  pub fn game_id(&self) -> Option<i64> { self.game_id.clone() }
  #[doc(hidden)] pub fn _set_game_id(&mut self, game_id: i64) -> &mut Self { self.game_id = Some(game_id); self }
  
  pub fn score(&self) -> Option<i32> { self.score.clone() }
  #[doc(hidden)] pub fn _set_score(&mut self, score: i32) -> &mut Self { self.score = Some(score); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



