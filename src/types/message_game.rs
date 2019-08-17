
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with a game. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageGame
  /// Game.
  game: Option<Game>,
  
}



impl Object for MessageGame {}
impl RObject for MessageGame {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageGame" }
  fn td_type(&self) -> RTDType { RTDType::MessageGame }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageGame {}


impl MessageGame {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageGame".to_string(),
      game: None,
      
    }
  }
  
  pub fn game(&self) -> Option<Game> { self.game.clone() }
  #[doc(hidden)] pub fn _set_game(&mut self, game: Game) -> &mut Self { self.game = Some(game); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



